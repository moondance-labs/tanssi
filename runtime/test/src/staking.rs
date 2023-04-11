use crate::{
	AccountId, Balance, Balances, Runtime, RuntimeEvent, Session,
	Staking, Timestamp, HOURS, MINUTES, SLOT_DURATION, UNIT,
};
use cumulus_primitives_core::relay_chain::{BlockNumber, LOWEST_PUBLIC_ID};
use frame_election_provider_support::{generate_solution_type, onchain, SequentialPhragmen};
use frame_support::{parameter_types, traits::EitherOf};
use frame_system::EnsureRoot;
use pallet_staking::UseValidatorsMap;
use polkadot_runtime_common::{paras_registrar::Paras, prod_or_fast};
use sp_core::ConstU32;
use sp_runtime::{curve::PiecewiseLinear, Perbill, Perquintill};
use sp_staking::SessionIndex;
use sp_std::vec;

pub const EPOCH_DURATION: u64 = 6 * HOURS;
pub const EPOCH_DURATION_IN_SLOTS: u64 = EPOCH_DURATION / SLOT_DURATION;

// Copied from Polkadot: https://github.com/paritytech/polkadot/blob/2c4627d8c63bcd9f08a5b025e44740928c4fbe19/runtime/polkadot/src/lib.rs#L506
parameter_types! {
	pub EpochDuration: u64 = prod_or_fast!(
		EPOCH_DURATION_IN_SLOTS as u64,
		2 * MINUTES as u64,
		"DOT_EPOCH_DURATION"
	);

	// phase durations. 1/4 of the last session for each.
	// in testing: 1min or half of the session for each
	pub SignedPhase: u32 = prod_or_fast!(
		EPOCH_DURATION_IN_SLOTS / 4,
		(1 * MINUTES).min(EpochDuration::get().saturated_into::<u32>() / 2),
		"DOT_SIGNED_PHASE"
	);
	pub UnsignedPhase: u32 = prod_or_fast!(
		EPOCH_DURATION_IN_SLOTS / 4,
		(1 * MINUTES).min(EpochDuration::get().saturated_into::<u32>() / 2),
		"DOT_UNSIGNED_PHASE"
	);

	// signed config
	pub const SignedMaxSubmissions: u32 = 16;
	pub const SignedMaxRefunds: u32 = 16 / 4;
	// 40 DOTs fixed deposit..
	pub const SignedDepositBase: Balance = deposit(2, 0);
	// 0.01 DOT per KB of solution data.
	pub const SignedDepositByte: Balance = deposit(0, 10) / 1024;
	// Each good submission will get 1 DOT as reward
	pub SignedRewardBase: Balance = 1 * UNIT;
	pub BetterUnsignedThreshold: Perbill = Perbill::from_rational(5u32, 10_000);

	// 4 hour session, 1 hour unsigned phase, 32 offchain executions.
	pub OffchainRepeat: BlockNumber = UnsignedPhase::get() / 32;

	/// We take the top 22500 nominators as electing voters..
	pub const MaxElectingVoters: u32 = 22_500;
	/// ... and all of the validators as electable targets. Whilst this is the case, we cannot and
	/// shall not increase the size of the validator intentions.
	pub const MaxElectableTargets: u16 = u16::MAX;
	/// Setup election pallet to support maximum winners upto 1200. This will mean Staking Pallet
	/// cannot have active validators higher than this count.
	pub const MaxActiveValidators: u32 = 1200;
}

pallet_staking_reward_curve::build! {
	const REWARD_CURVE: PiecewiseLinear<'static> = curve!(
		min_inflation: 0_025_000,
		max_inflation: 0_100_000,
		// 3:2:1 staked : parachains : float.
		// while there's no parachains, then this is 75% staked : 25% float.
		ideal_stake: 0_750_000,
		falloff: 0_050_000,
		max_piece_count: 40,
		test_precision: 0_005_000,
	);
}

generate_solution_type!(
	#[compact]
	pub struct NposCompactSolution16::<
		VoterIndex = u32,
		TargetIndex = u16,
		Accuracy = sp_runtime::PerU16,
		MaxVoters = MaxElectingVoters,
	>(16)
);

parameter_types! {
	// Six sessions in an era (24 hours).
	pub const SessionsPerEra: SessionIndex = prod_or_fast!(6, 1);

	// 28 eras for unbonding (28 days).
	pub BondingDuration: sp_staking::EraIndex = prod_or_fast!(
		28,
		28,
		"DOT_BONDING_DURATION"
	);
	pub SlashDeferDuration: sp_staking::EraIndex = prod_or_fast!(
		27,
		27,
		"DOT_SLASH_DEFER_DURATION"
	);
	pub const RewardCurve: &'static PiecewiseLinear<'static> = &REWARD_CURVE;
	pub const MaxNominatorRewardedPerValidator: u32 = 512;
	pub const OffendingValidatorsThreshold: Perbill = Perbill::from_percent(17);
	// 16
	pub const MaxNominations: u32 = <NposCompactSolution16 as frame_election_provider_support::NposSolution>::LIMIT as u32;
}

fn era_payout(
	total_staked: Balance,
	total_stakable: Balance,
	max_annual_inflation: Perquintill,
	period_fraction: Perquintill,
	auctioned_slots: u64,
) -> (Balance, Balance) {
	use pallet_staking_reward_fn::compute_inflation;
	use sp_runtime::traits::Saturating;

	let min_annual_inflation = Perquintill::from_rational(25u64, 1000u64);
	let delta_annual_inflation = max_annual_inflation.saturating_sub(min_annual_inflation);

	// 30% reserved for up to 60 slots.
	let auction_proportion = Perquintill::from_rational(auctioned_slots.min(60), 200u64);

	// Therefore the ideal amount at stake (as a percentage of total issuance) is 75% less the
	// amount that we expect to be taken up with auctions.
	let ideal_stake = Perquintill::from_percent(75).saturating_sub(auction_proportion);

	let stake = Perquintill::from_rational(total_staked, total_stakable);
	let falloff = Perquintill::from_percent(5);
	let adjustment = compute_inflation(stake, ideal_stake, falloff);
	let staking_inflation =
		min_annual_inflation.saturating_add(delta_annual_inflation * adjustment);

	let max_payout = period_fraction * max_annual_inflation * total_stakable;
	let staking_payout = (period_fraction * staking_inflation) * total_stakable;
	let rest = max_payout.saturating_sub(staking_payout);

	let other_issuance = total_stakable.saturating_sub(total_staked);
	if total_staked > other_issuance {
		let _cap_rest = Perquintill::from_rational(other_issuance, total_staked) * staking_payout;
		// We don't do anything with this, but if we wanted to, we could introduce a cap on the
		// treasury amount with: `rest = rest.min(cap_rest);`
	}
	(staking_payout, rest)
}

pub struct EraPayout;
impl pallet_staking::EraPayout<Balance> for EraPayout {
	fn era_payout(
		total_staked: Balance,
		total_issuance: Balance,
		era_duration_millis: u64,
	) -> (Balance, Balance) {
		// all para-ids that are not active.
		let auctioned_slots = Paras::parachains()
			.into_iter()
			// all active para-ids that do not belong to a system or common good chain is the number
			// of parachains that we should take into account for inflation.
			.filter(|i| *i >= LOWEST_PUBLIC_ID)
			.count() as u64;

		const MAX_ANNUAL_INFLATION: Perquintill = Perquintill::from_percent(10);
		const MILLISECONDS_PER_YEAR: u64 = 1000 * 3600 * 24 * 36525 / 100;

		era_payout(
			total_staked,
			total_issuance,
			MAX_ANNUAL_INFLATION,
			Perquintill::from_rational(era_duration_millis, MILLISECONDS_PER_YEAR),
			auctioned_slots,
		)
	}
}

// impl pallet_election_provider_multi_phase::MinerConfig for Runtime {
// 	type AccountId = AccountId;
// 	type MaxLength = OffchainSolutionLengthLimit;
// 	type MaxWeight = OffchainSolutionWeightLimit;
// 	type Solution = NposCompactSolution16;
// 	type MaxVotesPerVoter = <
// 		<Self as pallet_election_provider_multi_phase::Config>::DataProvider
// 		as
// 		frame_election_provider_support::ElectionDataProvider
// 	>::MaxVotesPerVoter;
// 	// type MaxWinners = MaxActiveValidators;

// 	// The unsigned submissions have to respect the weight of the submit_unsigned call, thus their
// 	// weight estimate function is wired to this call's weight.
// 	fn solution_weight(v: u32, t: u32, a: u32, d: u32) -> Weight {
// 		<
// 			<Self as pallet_election_provider_multi_phase::Config>::WeightInfo
// 			as
// 			pallet_election_provider_multi_phase::WeightInfo
// 		>::submit_unsigned(v, t, a, d)
// 	}
// }

// impl pallet_election_provider_multi_phase::Config for Runtime {
// 	type RuntimeEvent = RuntimeEvent;
// 	type Currency = Balances;
// 	type EstimateCallFee = TransactionPayment;
// 	type SignedPhase = SignedPhase;
// 	type UnsignedPhase = UnsignedPhase;
// 	type SignedMaxSubmissions = SignedMaxSubmissions;
// 	type SignedMaxRefunds = SignedMaxRefunds;
// 	type SignedRewardBase = SignedRewardBase;
// 	type SignedDepositBase = SignedDepositBase;
// 	type SignedDepositByte = SignedDepositByte;
// 	type SignedDepositWeight = ();
// 	type SignedMaxWeight =
// 		<Self::MinerConfig as pallet_election_provider_multi_phase::MinerConfig>::MaxWeight;
// 	type MinerConfig = Self;
// 	type SlashHandler = (); // burn slashes
// 	type RewardHandler = (); // nothing to do upon rewards
// 	type BetterUnsignedThreshold = BetterUnsignedThreshold;
// 	type BetterSignedThreshold = ();
// 	type OffchainRepeat = OffchainRepeat;
// 	type MinerTxPriority = NposSolutionPriority;
// 	type DataProvider = Staking;
// 	#[cfg(feature = "fast-runtime")]
// 	type Fallback = onchain::OnChainExecution<OnChainSeqPhragmen>;
// 	#[cfg(not(feature = "fast-runtime"))]
// 	type Fallback = frame_election_provider_support::NoElection<(
// 		AccountId,
// 		BlockNumber,
// 		Staking,
// 		MaxActiveValidators,
// 	)>;
// 	type GovernanceFallback = onchain::OnChainExecution<OnChainSeqPhragmen>;
// 	type Solver = SequentialPhragmen<
// 		AccountId,
// 		pallet_election_provider_multi_phase::SolutionAccuracyOf<Self>,
// 		(),
// 	>;
// 	type BenchmarkingConfig = BenchmarkConfig;
// 	type ForceOrigin = EitherOf<EnsureRoot<Self::AccountId>, StakingAdmin>;
// 	type WeightInfo = (); //weights::pallet_election_provider_multi_phase::WeightInfo<Self>;
// 	type MaxElectingVoters = MaxElectingVoters;
// 	type MaxElectableTargets = MaxElectableTargets;
// 	type MaxWinners = MaxActiveValidators;
// }


pub type OnChainAccuracy = sp_runtime::Perbill;

pub struct OnChainSeqPhragmen;
impl onchain::Config for OnChainSeqPhragmen {
	type System = Runtime;
	type Solver = SequentialPhragmen<AccountId, OnChainAccuracy>;
	type DataProvider = Staking;
	type WeightInfo = (); // weights::frame_election_provider_support::WeightInfo<Runtime>;
	type MaxWinners = MaxActiveValidators;
	type VotersBound = MaxElectingVoters;
	type TargetsBound = MaxElectableTargets;
}

pub type CurrencyToVote = frame_support::traits::U128CurrencyToVote;
static_assertions::assert_eq_size!(Balance, u128);

/// A reasonable benchmarking config for staking pallet.
pub struct StakingBenchmarkingConfig;
impl pallet_staking::BenchmarkingConfig for StakingBenchmarkingConfig {
	type MaxValidators = ConstU32<1000>;
	type MaxNominators = ConstU32<1000>;
}

impl pallet_staking::Config for Runtime {
	type MaxNominations = MaxNominations;
	type Currency = Balances;
	type CurrencyBalance = Balance;
	type UnixTime = Timestamp;
	type CurrencyToVote = CurrencyToVote;
	type RewardRemainder = (); // Polkadot: Treasury
	type RuntimeEvent = RuntimeEvent;
	type Slash = (); // Polkadot: Treasury
	type Reward = ();
	type SessionsPerEra = SessionsPerEra;
	type BondingDuration = BondingDuration;
	type SlashDeferDuration = SlashDeferDuration;
	type AdminOrigin = EitherOf<EnsureRoot<Self::AccountId>, StakingAdmin>;
	type SessionInterface = Self;
	type EraPayout = EraPayout;
	type MaxNominatorRewardedPerValidator = MaxNominatorRewardedPerValidator;
	type OffendingValidatorsThreshold = OffendingValidatorsThreshold;
	type NextNewSession = Session;
	type ElectionProvider = ElectionProviderMultiPhase;
	type GenesisElectionProvider = onchain::OnChainExecution<OnChainSeqPhragmen>;
	type VoterList = VoterList;
	type TargetList = UseValidatorsMap<Self>;
	type MaxUnlockingChunks = frame_support::traits::ConstU32<32>;
	type HistoryDepth = frame_support::traits::ConstU32<84>;
	type BenchmarkingConfig = StakingBenchmarkingConfig;
	type OnStakerSlash = (); // NominationPools;
	type WeightInfo = (); // TODO: weights::pallet_staking::WeightInfo<Runtime>
}

pub use pallet_custom_origins::*;

#[frame_support::pallet]
pub mod pallet_custom_origins {
	use crate::{Balance, KILO_UNIT, UNIT};
	use frame_support::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[derive(PartialEq, Eq, Clone, MaxEncodedLen, Encode, Decode, TypeInfo, RuntimeDebug)]
	#[pallet::origin]
	pub enum Origin {
		/// Origin able to cancel slashes and manage minimum commission.
		StakingAdmin,
		/// Origin for spending up to $10,000,000 DOT from the treasury as well as generally
		/// administering it.
		Treasurer,
		/// Origin for managing the composition of the fellowship.
		FellowshipAdmin,
		/// Origin for managing the registrar.
		GeneralAdmin,
		/// Origin for starting auctions.
		AuctionAdmin,
		/// Origin able to force slot leases.
		LeaseAdmin,
		/// Origin able to cancel referenda.
		ReferendumCanceller,
		/// Origin able to kill referenda.
		ReferendumKiller,
		/// Origin able to spend around $250 from the treasury at once.
		SmallTipper,
		/// Origin able to spend around $1,000 from the treasury at once.
		BigTipper,
		/// Origin able to spend around $10,000 from the treasury at once.
		SmallSpender,
		/// Origin able to spend around $100,000 from the treasury at once.
		MediumSpender,
		/// Origin able to spend up to $1,000,000 DOT from the treasury at once.
		BigSpender,
		/// Origin able to dispatch a whitelisted call.
		WhitelistedCaller,
	}

	macro_rules! decl_unit_ensures {
		( $name:ident: $success_type:ty = $success:expr ) => {
			pub struct $name;
			impl<O: Into<Result<Origin, O>> + From<Origin>>
				EnsureOrigin<O> for $name
			{
				type Success = $success_type;
				fn try_origin(o: O) -> Result<Self::Success, O> {
					o.into().and_then(|o| match o {
						Origin::$name => Ok($success),
						r => Err(O::from(r)),
					})
				}
				#[cfg(feature = "runtime-benchmarks")]
				fn try_successful_origin() -> Result<O, ()> {
					Ok(O::from(Origin::$name))
				}
			}
		};
		( $name:ident ) => { decl_unit_ensures! { $name : () = () } };
		( $name:ident: $success_type:ty = $success:expr, $( $rest:tt )* ) => {
			decl_unit_ensures! { $name: $success_type = $success }
			decl_unit_ensures! { $( $rest )* }
		};
		( $name:ident, $( $rest:tt )* ) => {
			decl_unit_ensures! { $name }
			decl_unit_ensures! { $( $rest )* }
		};
		() => {}
	}
	decl_unit_ensures!(
		StakingAdmin,
		Treasurer,
		FellowshipAdmin,
		GeneralAdmin,
		AuctionAdmin,
		LeaseAdmin,
		ReferendumCanceller,
		ReferendumKiller,
		WhitelistedCaller,
	);

	macro_rules! decl_ensure {
		(
			$vis:vis type $name:ident: EnsureOrigin<Success = $success_type:ty> {
				$( $item:ident = $success:expr, )*
			}
		) => {
			$vis struct $name;
			impl<O: Into<Result<Origin, O>> + From<Origin>>
				EnsureOrigin<O> for $name
			{
				type Success = $success_type;
				fn try_origin(o: O) -> Result<Self::Success, O> {
					o.into().and_then(|o| match o {
						$(
							Origin::$item => Ok($success),
						)*
						r => Err(O::from(r)),
					})
				}
				#[cfg(feature = "runtime-benchmarks")]
				fn try_successful_origin() -> Result<O, ()> {
					// By convention the more privileged origins go later, so for greatest chance
					// of success, we want the last one.
					let _result: Result<O, ()> = Err(());
					$(
						let _result: Result<O, ()> = Ok(O::from(Origin::$item));
					)*
					_result
				}
			}
		}
	}

	decl_ensure! {
		pub type Spender: EnsureOrigin<Success = Balance> {
			SmallTipper = 250 * UNIT,
			BigTipper = 1 * KILO_UNIT,
			SmallSpender = 10 * KILO_UNIT,
			MediumSpender = 100 * KILO_UNIT,
			BigSpender = 1_000 * KILO_UNIT,
			Treasurer = 10_000 * KILO_UNIT,
		}
	}
}
