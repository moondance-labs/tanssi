// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
pub use weights::WeightInfo;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;
pub mod weights;
pub use weights::*;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

use frame_support::sp_runtime::{
	traits::{AccountIdConversion, StaticLookup, Zero},
	Permill, RuntimeDebug,
};

use enumflags2::BitFlags;

use frame_support::sp_runtime::Percent;

use frame_support::{
	//	inherent::Vec,
	pallet_prelude::*,
	traits::{
		Currency, ExistenceRequirement::KeepAlive, Get, Incrementable, OnUnbalanced,
		ReservableCurrency, UnixTime,
	},
	PalletId,
};

pub use pallet_nfts::{
	CollectionConfig, CollectionSetting, CollectionSettings, ItemConfig, ItemSettings, MintSettings,
};

pub use sp_std::prelude::*;

type AccountIdLookupOf<T> = <<T as frame_system::Config>::Lookup as StaticLookup>::Source;

pub type LoanApy = u64;

pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

pub type NegativeImbalanceOf<T> = <<T as Config>::Currency as Currency<
	<T as frame_system::Config>::AccountId,
>>::NegativeImbalance;

pub type PositiveImbalanceOf<T> = <<T as Config>::Currency as Currency<
	<T as frame_system::Config>::AccountId,
>>::PositiveImbalance;

pub type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

pub type BalanceOf1<T> = <<T as pallet_nfts::Config>::Currency as Currency<
	<T as frame_system::Config>::AccountId,
>>::Balance;

pub type BoundedProposedMilestones<T> =
	BoundedVec<ProposedMilestone, <T as Config>::MaxMilestonesPerProject>;

pub const BASEINTERESTRATE: u32 = 525;

#[cfg(feature = "runtime-benchmarks")]
pub struct NftHelper;

#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[derive(Encode, Decode, Clone, PartialEq, Eq, MaxEncodedLen, RuntimeDebug, TypeInfo)]
pub struct ProposedMilestone {
	pub percentage_to_unlock: Percent,
}

#[cfg(feature = "runtime-benchmarks")]
pub trait BenchmarkHelper<CollectionId, ItemId> {
	fn to_collection(i: u32) -> CollectionId;
	fn to_nft(i: u32) -> ItemId;
}

#[cfg(feature = "runtime-benchmarks")]
impl<CollectionId: From<u32>, ItemId: From<u32>> BenchmarkHelper<CollectionId, ItemId>
	for NftHelper
{
	fn to_collection(i: u32) -> CollectionId {
		i.into()
	}
	fn to_nft(i: u32) -> ItemId {
		i.into()
	}
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::sp_runtime::Saturating;
	use frame_system::pallet_prelude::*;
	use scale_info::TypeInfo;

	pub type ProposalIndex = u32;
	pub type LoanIndex = u32;

	/// Loan proposal with the details.
	#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
	#[derive(Encode, Decode, Clone, PartialEq, Eq, MaxEncodedLen, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct Proposal<Balance, BlockNumber, T: Config> {
		proposer: AccountIdOf<T>,
		amount: Balance,
		milestones: BoundedProposedMilestones<T>,
		beneficiary: AccountIdOf<T>,
		apr_rate: LoanApy,
		bond: Balance,
		created_at: BlockNumber,
	}

	/// Infos from milestone proposals.
	#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
	#[derive(Encode, Decode, Clone, PartialEq, Eq, MaxEncodedLen, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct MilestoneProposalInfo<Balance, T: Config> {
		proposer: AccountIdOf<T>,
		bond: Balance,
	}

	/// Infos for loan.
	#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
	#[derive(Encode, Decode, Clone, PartialEq, Eq, MaxEncodedLen, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct LoanInfo<Balance, CollectionId, ItemId, T: Config> {
		pub borrower: AccountIdOf<T>,
		/// The loan amount borrowed at the beginning
		pub loan_amount: Balance,
		/// The loan amount less the repayment installments
		pub current_loan_balance: Balance,
		/// The loan amount that is currently available for the real estate developer.
		pub available_amount: Balance,
		/// The loan amount that has been borrowed.
		pub borrowed_amount: Balance,
		pub charged_interests: Balance,
		pub milestones: BoundedProposedMilestones<T>,
		pub collection_id: CollectionId,
		pub item_id: ItemId,
		pub loan_apy: LoanApy,
		pub last_timestamp: u64,
		pub withdraw_lock: bool,
	}

	/// AccountId storage.
	#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
	#[derive(Encode, Decode, Clone, PartialEq, Eq, MaxEncodedLen, RuntimeDebug, TypeInfo)]
	pub struct PalletIdStorage<T: Config> {
		pallet_id: AccountIdOf<T>,
	}

	/// Vote enum.
	#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
	#[derive(Encode, Decode, Clone, PartialEq, Eq, MaxEncodedLen, RuntimeDebug, TypeInfo)]
	pub enum Vote {
		Yes,
		No,
	}

	/// Voting stats.
	#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
	#[derive(Encode, Decode, Clone, PartialEq, Eq, MaxEncodedLen, RuntimeDebug, TypeInfo)]
	pub struct VoteStats {
		pub yes_votes: u64,
		pub no_votes: u64,
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config:
		frame_system::Config + pallet_nfts::Config + pallet_xcavate_whitelist::Config
	{
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// The currency type.
		type Currency: Currency<AccountIdOf<Self>> + ReservableCurrency<AccountIdOf<Self>>;

		/// Origin who can add or remove committee members.
		type CommitteeOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		/// Fraction of a proposal's value that should be bonded in order to place the proposal.
		/// An accepted proposal gets these back. A rejected proposal does not.
		#[pallet::constant]
		type ProposalBond: Get<Permill>;

		/// Minimum amount of funds that should be placed in a deposit for making a proposal.
		#[pallet::constant]
		type ProposalBondMinimum: Get<BalanceOf<Self>>;

		/// Maximum amount of funds that should be placed in a deposit for making a proposal.
		#[pallet::constant]
		type ProposalBondMaximum: Get<Option<BalanceOf<Self>>>;

		/// The community-loan-pool's pallet id, used for deriving its sovereign account ID.
		#[pallet::constant]
		type PalletId: Get<PalletId>;

		/// Handler for the unbalanced decrease when slashing for a rejected proposal or bounty.
		type OnSlash: OnUnbalanced<NegativeImbalanceOf<Self>>;

		/// The maximum amount of loans that can run at the same time.
		#[pallet::constant]
		type MaxOngoingLoans: Get<u32>;

		/// lose coupling of pallet timestamp.
		type TimeProvider: UnixTime;

		/// Type representing the weight of this pallet.
		type WeightInfo: WeightInfo;

		#[cfg(feature = "runtime-benchmarks")]
		type Helper: crate::BenchmarkHelper<
			<Self as pallet::Config>::CollectionId,
			<Self as pallet::Config>::ItemId,
		>;

		/// The amount of time given to vote for a proposal.
		type VotingTime: Get<BlockNumberFor<Self>>;

		/// The maximum amount of commitee members.
		type MaxCommitteeMembers: Get<u32>;

		/// The maximum amount of milestones for a lone.
		type MaxMilestonesPerProject: Get<u32>;

		type CollectionId: IsType<<Self as pallet_nfts::Config>::CollectionId>
			+ Parameter
			+ From<u32>
			+ Ord
			+ Copy
			+ MaxEncodedLen
			+ Encode;

		type ItemId: IsType<<Self as pallet_nfts::Config>::ItemId>
			+ Parameter
			+ From<u32>
			+ Ord
			+ Copy
			+ MaxEncodedLen
			+ Encode;
	}

	pub type CollectionId<T> = <T as Config>::CollectionId;
	pub type ItemId<T> = <T as Config>::ItemId;

	pub(super) type LoanInfoType<T> = LoanInfo<
		BalanceOf<T>,
		<T as pallet::Config>::CollectionId,
		<T as pallet::Config>::ItemId,
		T,
	>;

	/// Vec of admins who are able to vote.
	#[pallet::storage]
	#[pallet::getter(fn voting_committee)]
	pub(super) type VotingCommittee<T: Config> =
		StorageValue<_, BoundedVec<AccountIdOf<T>, T::MaxCommitteeMembers>, ValueQuery>;

	/// Number of loans that have been made.
	#[pallet::storage]
	#[pallet::getter(fn loan_count)]
	pub(super) type LoanCount<T> = StorageValue<_, LoanIndex, ValueQuery>;

	/// Number of proposals that have been made.
	#[pallet::storage]
	#[pallet::getter(fn proposal_count)]
	pub(super) type ProposalCount<T> = StorageValue<_, ProposalIndex, ValueQuery>;

	/// Number of milestone proposals that have been made.
	#[pallet::storage]
	#[pallet::getter(fn milestone_proposal_count)]
	pub(super) type MilestoneProposalCount<T> = StorageValue<_, ProposalIndex, ValueQuery>;

	/// Number of deletion proposals that have been made.
	#[pallet::storage]
	#[pallet::getter(fn deletion_proposal_count)]
	pub(super) type DeletionProposalCount<T> = StorageValue<_, ProposalIndex, ValueQuery>;

	/// Total amount of loan funds.
	#[pallet::storage]
	#[pallet::getter(fn total_loan_amount)]
	pub(super) type TotalLoanAmount<T> = StorageValue<_, u64, ValueQuery>;

	/// Total amount of not paid interests.
	#[pallet::storage]
	#[pallet::getter(fn total_loan_interests)]
	pub(super) type TotalLoanInterests<T> = StorageValue<_, u64, ValueQuery>;

	/// Amount of founds that is still on the pallet but is reserved for loan.
	#[pallet::storage]
	#[pallet::getter(fn reserved_loan_amount)]
	pub(super) type ReservedLoanAmount<T> = StorageValue<_, u64, ValueQuery>;

	/// All currently ongoing loans.
	#[pallet::storage]
	#[pallet::getter(fn ongoing_loans)]
	pub(super) type OngoingLoans<T: Config> =
		StorageValue<_, BoundedVec<ProposalIndex, T::MaxOngoingLoans>, ValueQuery>;

	/// Proposals that have been made.
	#[pallet::storage]
	#[pallet::getter(fn proposals)]
	pub(super) type Proposals<T> = StorageMap<
		_,
		Blake2_128Concat,
		ProposalIndex,
		Proposal<BalanceOf<T>, BlockNumberFor<T>, T>,
		OptionQuery,
	>;

	/// Milestone proposals that has been made.
	#[pallet::storage]
	#[pallet::getter(fn milestone_proposals)]
	pub(super) type MilestoneProposals<T> =
		StorageMap<_, Blake2_128Concat, ProposalIndex, LoanIndex, OptionQuery>;

	/// Milestone proposal that has been made.
	#[pallet::storage]
	#[pallet::getter(fn milestone_info)]
	pub(super) type MilestoneInfo<T> = StorageMap<
		_,
		Blake2_128Concat,
		ProposalIndex,
		MilestoneProposalInfo<BalanceOf<T>, T>,
		OptionQuery,
	>;

	/// Deletion proposals that have been made.
	#[pallet::storage]
	#[pallet::getter(fn deletion_proposals)]
	pub(super) type DeletionProposals<T> =
		StorageMap<_, Blake2_128Concat, ProposalIndex, LoanIndex, OptionQuery>;

	/// Mapping of ongoing loans.
	#[pallet::storage]
	#[pallet::getter(fn loans)]
	pub(super) type Loans<T: Config> =
		StorageMap<_, Blake2_128Concat, LoanIndex, LoanInfoType<T>, OptionQuery>;

	/// Mapping of ongoing votes.
	#[pallet::storage]
	#[pallet::getter(fn ongoing_votes)]
	pub(super) type OngoingVotes<T: Config> =
		StorageMap<_, Blake2_128Concat, ProposalIndex, VoteStats, OptionQuery>;

	/// Mapping of ongoing milstone votes.
	#[pallet::storage]
	#[pallet::getter(fn ongoing_milestone_votes)]
	pub(super) type OngoingMilestoneVotes<T: Config> =
		StorageMap<_, Blake2_128Concat, ProposalIndex, VoteStats, OptionQuery>;

	/// Mapping of ongoing deletion votes.
	#[pallet::storage]
	#[pallet::getter(fn ongoing_deletion_votes)]
	pub(super) type OngoingDeletionVotes<T: Config> =
		StorageMap<_, Blake2_128Concat, ProposalIndex, VoteStats, OptionQuery>;

	/// Mapping of user who voted for a proposal.
	#[pallet::storage]
	#[pallet::getter(fn user_votes)]
	pub(super) type UserVotes<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		ProposalIndex,
		Blake2_128Concat,
		AccountIdOf<T>,
		Vote,
		OptionQuery,
	>;

	/// Mapping of user who voted for a milestone proposal.
	#[pallet::storage]
	#[pallet::getter(fn user_milestone_votes)]
	pub(super) type UserMilestoneVotes<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		ProposalIndex,
		Blake2_128Concat,
		AccountIdOf<T>,
		Vote,
		OptionQuery,
	>;

	/// Mapping of user who voted for a deletion proposal.
	#[pallet::storage]
	#[pallet::getter(fn user_deletion_votes)]
	pub(super) type UserDeletionVotes<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		ProposalIndex,
		Blake2_128Concat,
		AccountIdOf<T>,
		Vote,
		OptionQuery,
	>;

	/// Stores the project keys and round types ending on a given block.
	#[pallet::storage]
	pub type RoundsExpiring<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BlockNumberFor<T>,
		BoundedVec<ProposalIndex, T::MaxOngoingLoans>,
		ValueQuery,
	>;

	/// Stores the project keys and round types ending on a given block for milestone votings.
	#[pallet::storage]
	pub type MilestoneRoundsExpiring<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BlockNumberFor<T>,
		BoundedVec<ProposalIndex, T::MaxOngoingLoans>,
		ValueQuery,
	>;

	/// Stores the project keys and round types ending on a given block for deletion votings.
	#[pallet::storage]
	pub type DeletionRoundsExpiring<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BlockNumberFor<T>,
		BoundedVec<ProposalIndex, T::MaxOngoingLoans>,
		ValueQuery,
	>;

	#[pallet::genesis_config]
	#[derive(frame_support::DefaultNoBound)]
	pub struct GenesisConfig<T: Config> {
		#[serde(skip)]
		_config: sp_std::marker::PhantomData<T>,
	}

	#[pallet::genesis_build]
	impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
		fn build(&self) {
			// Create community loan pool account
			let account_id = <Pallet<T>>::account_id();
			let min = <T as pallet::Config>::Currency::minimum_balance();
			if <T as pallet::Config>::Currency::free_balance(&account_id) < min {
				let _ = <T as pallet::Config>::Currency::make_free_balance_be(&account_id, min);
			}
		}
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Proposer's balance is too low.
		InsufficientProposersBalance,
		/// Loan pool's balance is too low.
		InsufficientLoanPoolBalance,
		/// No proposal index.
		InvalidIndex,
		/// The caller doesn't have enough permission.
		InsufficientPermission,
		/// Max amount of ongoing loan reached.
		TooManyLoans,
		/// User has already voted.
		AlreadyVoted,
		/// Loan got not approved.
		NotApproved,
		/// The account is already a member in the voting committee.
		AlreadyMember,
		/// There are already enough committee members.
		TooManyMembers,
		/// There are not enough funds available in the loan.
		NotEnoughFundsToWithdraw,
		/// The loan is still ongoing.
		LoanStillOngoing,
		/// All milestones have been accomplished.
		NoMilestonesLeft,
		/// Milestones of the loan have to be 100 % in sum
		MilestonesHaveToCoverLoan,
		/// Withdrawl is locked during ongoing voting for deletion.
		DeletionVotingOngoing,
		/// The beneficiary didn't borrow that much funds.
		WantsToRepayTooMuch,
		/// There are not enough funds available in the loan pallet.
		NotEnoughLoanFundsAvailable,
		/// The Milestones for the proposal have already been set.
		MilestonesAlreadySet,
		/// There has been no milestones set in the proposal.
		NoMilestones,
		/// There is an issue by calling the next collection id.
		UnknownCollection,
		/// Error by convertion to balance type.
		ConversionError,
		ArithmeticUnderflow,
		/// Error by dividing a number.
		DivisionError,
		/// This loan does not exist
		NoLoanFound,
		/// User has not passed the kyc.
		UserNotWhitelisted,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// New proposal has been created.
		Proposed { proposal_index: ProposalIndex },
		/// New milestone proposal has been created.
		MilestoneProposed { proposal_index: ProposalIndex },
		/// New deletion proposal has been created.
		DeletionProposed { proposal_index: ProposalIndex, loan_index: LoanIndex },
		/// Proposal has been approved.
		Approved { proposal_index: ProposalIndex },
		/// Proposal has been rejected.
		Rejected { proposal_index: ProposalIndex },
		/// Loan has been deleted.
		Deleted { loan_index: LoanIndex },
		/// Apy has been charged.
		ApyCharged { loan_index: LoanIndex, interest_balance: BalanceOf<T> },
		/// Loan has been updated.
		LoanUpdated { loan_index: LoanIndex },
		/// User withdrew money.
		Withdraw { loan_index: LoanIndex, amount: BalanceOf<T> },
		/// Voted on a proposal.
		VotedOnProposal { proposal_index: ProposalIndex, member: AccountIdOf<T>, vote: Vote },
		/// Voted on a milestone.
		VotedOnMilestone { proposal_index: ProposalIndex, member: AccountIdOf<T>, vote: Vote },
		/// Voted on a deletion.
		VotedOnDeletion { proposal_index: ProposalIndex, member: AccountIdOf<T>, vote: Vote },
		/// A new committee member has been added.
		CommiteeMemberAdded { new_member: AccountIdOf<T> },
		/// Milestone Proposal has been approved.
		MilestoneApproved { loan_id: LoanIndex },
		/// Milestone Proposal has been rejected.
		MilestoneRejected { proposal_index: ProposalIndex },
		/// Milestones have been set for a proposal.
		MilestonesSet { proposal_index: ProposalIndex },
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(n: frame_system::pallet_prelude::BlockNumberFor<T>) -> Weight {
			let mut weight = T::DbWeight::get().reads_writes(1, 1);

			let ended_votings = RoundsExpiring::<T>::take(n);

			// Checks if there is a voting for a loan expiring in this block.
			ended_votings.iter().for_each(|item| {
				weight = weight.saturating_add(T::DbWeight::get().reads_writes(1, 1));
				let voting_result = <OngoingVotes<T>>::take(item);
				if let Some(voting_result) = voting_result {
					if voting_result.yes_votes > voting_result.no_votes {
						let _ = Self::approve_loan_proposal(*item);
					} else {
						let _ = Self::reject_loan_proposal(*item);
					}
					OngoingVotes::<T>::remove(item);
				}
			});

			let ended_milestone_votes = MilestoneRoundsExpiring::<T>::take(n);

			// checks if there is a voting for a milestone expiring in this block.
			ended_milestone_votes.iter().for_each(|item| {
				weight = weight.saturating_add(T::DbWeight::get().reads_writes(1, 1));
				let voting_result = <OngoingMilestoneVotes<T>>::take(item);
				if let Some(voting_result) = voting_result {
					if voting_result.yes_votes > voting_result.no_votes {
						let loan_id = <MilestoneProposals<T>>::take(item);
						if let Some(loan_id) = loan_id {
							let _ = Self::updating_available_amount(loan_id, item);
						}
					} else {
						let _ = Self::reject_milestone(item);
					}
					OngoingMilestoneVotes::<T>::remove(item);
				}
			});

			let ended_deletion_votes = DeletionRoundsExpiring::<T>::take(n);

			// checks if there is a voting for a loan deletion expiring in this block.
			ended_deletion_votes.iter().for_each(|item| {
				weight = weight.saturating_add(T::DbWeight::get().reads_writes(1, 1));
				let voting_result = <OngoingDeletionVotes<T>>::take(item);
				let loan_id = <DeletionProposals<T>>::take(item);
				if let Some(voting_result) = voting_result {
					if voting_result.yes_votes > voting_result.no_votes {
						if let Some(loan_id) = loan_id {
							let _ = Self::delete_loan(loan_id);
						}
					} else if let Some(loan_id) = loan_id {
						let _ = Self::open_withdrawl(loan_id);
					}

					OngoingDeletionVotes::<T>::remove(item);
				}
			});
			// Charging loan apy every block for testing purpose
			//let block = n.saturated_into::<u64>();
			//if block % 10 == 0 {
			Self::charge_apy().unwrap_or_default();
			weight.saturating_add(T::DbWeight::get().reads_writes(1, 1));
			//}

			weight
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Creates a proposal for a loan. A deposit amount is reserved
		/// and slashed if the proposal is rejected. It is returned once the proposal is awarded.
		///
		/// The origin must be Signed and the sender must have sufficient funds free.
		///
		/// Parameters:
		/// - `amount`: The amount of token that the real estate developer wants to lend.
		/// - `beneficiary`: The account that should be able to receive the funds.
		/// - `developer_experience`: Amout of years that the real estate developer has in experience.
		/// - `loan_term`: Estimated duration of the loan in months.
		///
		/// Emits `Proposed` event when succesfful
		#[pallet::call_index(0)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::propose())]
		pub fn propose(
			origin: OriginFor<T>,
			amount: BalanceOf<T>,
			beneficiary: AccountIdLookupOf<T>,
			developer_experience: u64,
			loan_term: u64,
		) -> DispatchResult {
			let origin = ensure_signed(origin)?;
			ensure!(
				pallet_xcavate_whitelist::Pallet::<T>::whitelisted_accounts(origin.clone()),
				Error::<T>::UserNotWhitelisted
			);
			let beneficiary = T::Lookup::lookup(beneficiary)?;
			let reserved_loan_amount = Self::u64_to_balance_option(Self::reserved_loan_amount())?;
			//let decimal = 1000000000000_u64.saturated_into();
			ensure!(
				<T as pallet::Config>::Currency::free_balance(&Self::account_id())
					>= reserved_loan_amount.saturating_add(amount),
				Error::<T>::NotEnoughLoanFundsAvailable
			);
			let proposal_index = Self::proposal_count() + 1;
			let bond = Self::calculate_bond(amount);
			<T as pallet::Config>::Currency::reserve(&origin, bond)
				.map_err(|_| Error::<T>::InsufficientProposersBalance)?;
			let current_block_number = <frame_system::Pallet<T>>::block_number();
			let expiry_block =
				current_block_number.saturating_add(<T as Config>::VotingTime::get());

			RoundsExpiring::<T>::try_mutate(expiry_block, |keys| {
				keys.try_push(proposal_index).map_err(|_| Error::<T>::TooManyLoans)?;
				Ok::<(), DispatchError>(())
			})?;
			let apr_rate = Self::calculate_apr(developer_experience, loan_term);
			let proposal = Proposal {
				proposer: origin,
				amount,
				milestones: Default::default(),
				beneficiary,
				apr_rate,
				bond,
				created_at: current_block_number,
			};
			let vote_stats = VoteStats { yes_votes: 0, no_votes: 0 };
			let reserved_value =
				Self::reserved_loan_amount().saturating_add(Self::balance_to_u64(amount)?);
			ReservedLoanAmount::<T>::put(reserved_value);
			OngoingVotes::<T>::insert(proposal_index, vote_stats);
			Proposals::<T>::insert(proposal_index, proposal);
			ProposalCount::<T>::put(proposal_index);

			Self::deposit_event(Event::Proposed { proposal_index });
			Ok(())
		}

		/// Creates a proposal for the next milestone in the loan.
		///
		/// The origin must be Signed and the sender must have sufficient funds free.
		///
		/// Parameters:
		/// - `loan_id`: The index of the loan.
		///
		/// Emits `MilestoneProposed` event when succesfful
		#[pallet::call_index(1)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::propose_milestone())]
		pub fn propose_milestone(origin: OriginFor<T>, loan_id: LoanIndex) -> DispatchResult {
			let origin = ensure_signed(origin)?;
			ensure!(
				pallet_xcavate_whitelist::Pallet::<T>::whitelisted_accounts(origin.clone()),
				Error::<T>::UserNotWhitelisted
			);
			let loan = Self::loans(loan_id).ok_or(Error::<T>::InvalidIndex)?;
			ensure!(loan.milestones.len() > 0, Error::<T>::NoMilestonesLeft);
			let milestone_proposal_index = Self::milestone_proposal_count() + 1;
			let current_block_number = <frame_system::Pallet<T>>::block_number();
			let expiry_block =
				current_block_number.saturating_add(<T as Config>::VotingTime::get());

			MilestoneRoundsExpiring::<T>::try_mutate(expiry_block, |keys| {
				keys.try_push(milestone_proposal_index).map_err(|_| Error::<T>::TooManyLoans)?;
				Ok::<(), DispatchError>(())
			})?;
			let vote_stats = VoteStats { yes_votes: 0, no_votes: 0 };
			OngoingMilestoneVotes::<T>::insert(milestone_proposal_index, vote_stats);
			let amount = loan.milestones[0].percentage_to_unlock.deconstruct();
			let bond = Self::calculate_bond(amount.into());
			<T as pallet::Config>::Currency::reserve(&origin, bond)
				.map_err(|_| Error::<T>::InsufficientProposersBalance)?;

			let milestone_details = MilestoneProposalInfo { proposer: origin, bond };
			MilestoneInfo::<T>::insert(milestone_proposal_index, milestone_details);
			MilestoneProposals::<T>::insert(milestone_proposal_index, loan_id);
			MilestoneProposalCount::<T>::put(milestone_proposal_index);
			Self::deposit_event(Event::MilestoneProposed {
				proposal_index: milestone_proposal_index,
			});
			Ok(())
		}

		/// Creates a proposal for loan deletion.
		///
		/// The origin must be the borrower of the loan, Signed and the sender must have sufficient funds free.
		///
		/// Parameters:
		/// - `loan_id`: The index of the loan.
		///
		/// Emits `DeletionProposed` event when succesfful
		#[pallet::call_index(2)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::propose_deletion())]
		pub fn propose_deletion(origin: OriginFor<T>, loan_id: LoanIndex) -> DispatchResult {
			let origin = ensure_signed(origin.clone())?;
			ensure!(
				pallet_xcavate_whitelist::Pallet::<T>::whitelisted_accounts(origin.clone()),
				Error::<T>::UserNotWhitelisted
			);
			let mut loan = <Loans<T>>::take(loan_id).ok_or(Error::<T>::InvalidIndex)?;
			ensure!(origin == loan.borrower, Error::<T>::InsufficientPermission);
			ensure!(loan.borrowed_amount.is_zero(), Error::<T>::LoanStillOngoing);
			let deletion_proposal_index = Self::deletion_proposal_count() + 1;
			let current_block_number = <frame_system::Pallet<T>>::block_number();
			let expiry_block =
				current_block_number.saturating_add(<T as Config>::VotingTime::get());
			loan.withdraw_lock = true;
			DeletionRoundsExpiring::<T>::try_mutate(expiry_block, |keys| {
				keys.try_push(deletion_proposal_index).map_err(|_| Error::<T>::TooManyLoans)?;
				Ok::<(), DispatchError>(())
			})?;
			let vote_stats = VoteStats { yes_votes: 0, no_votes: 0 };
			OngoingDeletionVotes::<T>::insert(deletion_proposal_index, vote_stats);
			DeletionProposals::<T>::insert(deletion_proposal_index, loan_id);
			DeletionProposalCount::<T>::put(deletion_proposal_index);
			Loans::<T>::insert(loan_id, loan);
			Self::deposit_event(Event::<T>::DeletionProposed {
				proposal_index: deletion_proposal_index,
				loan_index: loan_id,
			});
			Ok(())
		}

		/// Lets the real estate developer withdraw the funds from the loan.
		///
		/// The origin must be the borrower of the loan, Signed and the sender must have sufficient funds free.
		///
		/// Parameters:
		/// - `loan_id`: The index of the loan.
		/// - `amount`: The amount of token the real estate developer wants to withdraw.
		///
		/// Emits `Withdraw` event when succesfful
		#[pallet::call_index(3)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::withdraw())]
		pub fn withdraw(
			origin: OriginFor<T>,
			loan_id: LoanIndex,
			amount: BalanceOf<T>,
		) -> DispatchResult {
			let signer = ensure_signed(origin.clone())?;
			ensure!(
				pallet_xcavate_whitelist::Pallet::<T>::whitelisted_accounts(signer.clone()),
				Error::<T>::UserNotWhitelisted
			);
			let mut loan = <Loans<T>>::take(loan_id).ok_or(Error::<T>::InvalidIndex)?;
			ensure!(signer == loan.borrower, Error::<T>::InsufficientPermission);
			ensure!(amount <= loan.available_amount, Error::<T>::NotEnoughFundsToWithdraw);
			ensure!(!loan.withdraw_lock, Error::<T>::DeletionVotingOngoing);
			let loan_pallet = Self::account_id();
			let sending_amount = Self::balance_to_u64(amount)?;
			// For unit tests the line with the
			<T as pallet::Config>::Currency::transfer(
				&loan_pallet,
				&signer,
				// For unit tests this line has to be commented out and the line blow has to be uncommented due to the dicmals on polkadot js
				(sending_amount as u128 * 1/* 000000000000 */)
					.try_into()
					.map_err(|_| Error::<T>::ConversionError)?,
				KeepAlive,
			)?;
			loan.borrowed_amount = loan.borrowed_amount.saturating_add(amount);
			loan.available_amount = loan.available_amount.saturating_sub(amount);
			Loans::<T>::insert(loan_id, loan);
			let reserved_value = Self::reserved_loan_amount()
				.checked_sub(Self::balance_to_u64(amount)?)
				.ok_or(Error::<T>::ArithmeticUnderflow)?;
			ReservedLoanAmount::<T>::put(reserved_value);
			Self::deposit_event(Event::<T>::Withdraw { loan_index: loan_id, amount });
			Ok(())
		}

		/// Lets the real estate developer repay the borrowed funds from the loan
		///
		/// The origin must be signed and the sender must have sufficient funds free.
		///
		/// Parameters:
		/// - `loan_id`: The index of the loan.
		/// - `amount`: The amount of token the real estate developer wants to repay
		///
		/// Emits `LoanUpdated` event when succesfful
		#[pallet::call_index(4)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::repay())]
		pub fn repay(
			origin: OriginFor<T>,
			loan_id: LoanIndex,
			amount: BalanceOf<T>,
		) -> DispatchResult {
			let signer = ensure_signed(origin.clone())?;
			ensure!(
				pallet_xcavate_whitelist::Pallet::<T>::whitelisted_accounts(signer.clone()),
				Error::<T>::UserNotWhitelisted
			);
			let mut loan = <Loans<T>>::take(loan_id).ok_or(Error::<T>::InvalidIndex)?;
			ensure!(amount <= loan.borrowed_amount, Error::<T>::WantsToRepayTooMuch);
			let loan_pallet = Self::account_id();
			let sending_amount = Self::balance_to_u64(amount)?;
			<T as pallet::Config>::Currency::transfer(
				&signer,
				&loan_pallet,
				// For unit tests this line has to be commented out and the line blow has to be uncommented due to the dicmals on polkadot js
				(sending_amount as u128 * 1/* 000000000000 */)
					.try_into()
					.map_err(|_| Error::<T>::ConversionError)?,
				KeepAlive,
			)?;
			loan.borrowed_amount = loan.borrowed_amount.saturating_sub(amount);
			if loan.current_loan_balance >= amount {
				loan.current_loan_balance = loan.current_loan_balance.saturating_sub(amount);
				Loans::<T>::insert(loan_id, loan);
				let new_value = Self::total_loan_amount()
					.checked_sub(Self::balance_to_u64(amount)?)
					.ok_or(Error::<T>::ArithmeticUnderflow)?;
				TotalLoanAmount::<T>::put(new_value);
			} else if loan.current_loan_balance.is_zero() {
				loan.charged_interests = loan.charged_interests.saturating_sub(amount);
				Loans::<T>::insert(loan_id, loan);
				let new_value = Self::total_loan_interests()
					.checked_sub(Self::balance_to_u64(amount)?)
					.ok_or(Error::<T>::ArithmeticUnderflow)?;
				TotalLoanInterests::<T>::put(new_value);
			} else {
				let loan_amount_part = loan.current_loan_balance;
				let interests_amount_part = amount.saturating_sub(loan_amount_part);
				loan.current_loan_balance =
					loan.current_loan_balance.saturating_sub(loan_amount_part);
				loan.charged_interests =
					loan.charged_interests.saturating_sub(interests_amount_part);
				Loans::<T>::insert(loan_id, loan);
				let new_value = Self::total_loan_amount()
					.checked_sub(Self::balance_to_u64(loan_amount_part)?)
					.ok_or(Error::<T>::ArithmeticUnderflow)?;
				TotalLoanAmount::<T>::put(new_value);
				let new_interests_value = Self::total_loan_interests()
					.checked_sub(Self::balance_to_u64(interests_amount_part)?)
					.ok_or(Error::<T>::ArithmeticUnderflow)?;
				TotalLoanInterests::<T>::put(new_interests_value);
			}
			Self::deposit_event(Event::<T>::LoanUpdated { loan_index: loan_id });
			Ok(())
		}

		/// Lets the committee set the milestones for a loan.
		///	The caller automaticly votes yes for the proposal by calling this function.
		///
		/// The origin must be a member of the committee, signed and the sender must have sufficient funds free.
		///
		/// Parameters:
		/// - `proposal_index`: The index of the proposal.
		/// - `proposed_milestones`: A vector with the different milestone percentages, it must be 100 in sum.
		///
		/// Emits `MilestonesSet` event when succesfful
		#[pallet::call_index(5)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::set_milestones())]
		pub fn set_milestones(
			origin: OriginFor<T>,
			proposal_index: ProposalIndex,
			proposed_milestones: BoundedProposedMilestones<T>,
		) -> DispatchResult {
			let origin = ensure_signed(origin)?;
			ensure!(
				pallet_xcavate_whitelist::Pallet::<T>::whitelisted_accounts(origin.clone()),
				Error::<T>::UserNotWhitelisted
			);
			let current_members = Self::voting_committee();
			ensure!(current_members.contains(&origin), Error::<T>::InsufficientPermission);
			let mut proposal =
				<Proposals<T>>::take(proposal_index).ok_or(Error::<T>::InvalidIndex)?;
			ensure!(proposal.milestones.len() == 0, Error::<T>::MilestonesAlreadySet);
			let sum: u64 = proposed_milestones
				.iter()
				.map(|i| i.percentage_to_unlock.deconstruct() as u64)
				.sum();
			ensure!(100 == sum, Error::<T>::MilestonesHaveToCoverLoan);
			proposal.milestones = proposed_milestones;
			Proposals::<T>::insert(proposal_index, proposal);
			let mut current_vote =
				<OngoingVotes<T>>::take(proposal_index).ok_or(Error::<T>::InvalidIndex)?;
			let voted = <UserVotes<T>>::get(proposal_index, origin.clone());
			if voted.is_none() {
				current_vote.yes_votes += 1;
				UserVotes::<T>::insert(proposal_index, origin.clone(), Vote::Yes);
				OngoingVotes::<T>::insert(proposal_index, current_vote);
				Self::deposit_event(Event::<T>::VotedOnProposal {
					proposal_index,
					member: origin,
					vote: Vote::Yes,
				});
			} else {
				OngoingVotes::<T>::insert(proposal_index, current_vote);
			};
			Self::deposit_event(Event::<T>::MilestonesSet { proposal_index });
			Ok(())
		}

		/// Let committee members vote for a proposal.
		///
		/// The origin must be a member of the committee, signed and the sender must have sufficient funds free.
		///
		/// Parameters:
		/// - `proposal_index`: The index of the proposal.
		/// - `vote`: Must be either a Yes vote or a No vote.
		///
		/// Emits `VotedOnProposal` event when succesfful.
		#[pallet::call_index(6)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::vote_on_proposal())]
		pub fn vote_on_proposal(
			origin: OriginFor<T>,
			proposal_index: ProposalIndex,
			vote: Vote,
		) -> DispatchResult {
			let origin = ensure_signed(origin)?;
			ensure!(
				pallet_xcavate_whitelist::Pallet::<T>::whitelisted_accounts(origin.clone()),
				Error::<T>::UserNotWhitelisted
			);
			let current_members = Self::voting_committee();
			ensure!(current_members.contains(&origin), Error::<T>::InsufficientPermission);
			let mut current_vote =
				<OngoingVotes<T>>::take(proposal_index).ok_or(Error::<T>::InvalidIndex)?;
			let voted = <UserVotes<T>>::get(proposal_index, origin.clone());
			ensure!(voted.is_none(), Error::<T>::AlreadyVoted);
			let proposal = Self::proposals(proposal_index).ok_or(Error::<T>::InvalidIndex)?;
			ensure!(proposal.milestones.len() > 0, Error::<T>::NoMilestones);
			if vote == Vote::Yes {
				current_vote.yes_votes += 1;
			} else {
				current_vote.no_votes += 1;
			};

			UserVotes::<T>::insert(proposal_index, origin.clone(), vote.clone());
			OngoingVotes::<T>::insert(proposal_index, current_vote);
			Self::deposit_event(Event::<T>::VotedOnProposal {
				proposal_index,
				member: origin,
				vote,
			});
			Ok(())
		}

		/// Let committee vote on milestone proposal.
		///
		/// The origin must be a member of the committee, signed and the sender must have sufficient funds free.
		///
		/// Parameters:
		/// - `proposal_index`: The index of the proposal.
		/// - `vote`: Must be either a Yes vote or a No vote.
		///
		/// Emits `VotedOnMilestone` event when succesfful.
		#[pallet::call_index(7)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::vote_on_milestone_proposal())]
		pub fn vote_on_milestone_proposal(
			origin: OriginFor<T>,
			proposal_index: ProposalIndex,
			vote: Vote,
		) -> DispatchResult {
			let origin = ensure_signed(origin)?;
			ensure!(
				pallet_xcavate_whitelist::Pallet::<T>::whitelisted_accounts(origin.clone()),
				Error::<T>::UserNotWhitelisted
			);
			let current_members = Self::voting_committee();
			ensure!(current_members.contains(&origin), Error::<T>::InsufficientPermission);
			let mut current_vote =
				<OngoingMilestoneVotes<T>>::take(proposal_index).ok_or(Error::<T>::InvalidIndex)?;
			let voted = <UserMilestoneVotes<T>>::get(proposal_index, origin.clone());
			ensure!(voted.is_none(), Error::<T>::AlreadyVoted);
			if vote == Vote::Yes {
				current_vote.yes_votes += 1;
			} else {
				current_vote.no_votes += 1;
			};

			UserMilestoneVotes::<T>::insert(proposal_index, origin.clone(), vote.clone());
			OngoingMilestoneVotes::<T>::insert(proposal_index, current_vote);
			Self::deposit_event(Event::<T>::VotedOnMilestone {
				proposal_index,
				member: origin,
				vote,
			});
			Ok(())
		}

		/// Let committee vote on deletion proposal.
		///
		/// The origin must be a member of the committee, signed and the sender must have sufficient funds free.
		///
		/// Parameters:
		/// - `proposal_index`: The index of the proposal.
		/// - `vote`: Must be either a Yes vote or a No vote.
		///
		/// Emits `VotedOnDeletion` event when succesfful.
		#[pallet::call_index(8)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::vote_on_deletion_proposal())]
		pub fn vote_on_deletion_proposal(
			origin: OriginFor<T>,
			proposal_index: ProposalIndex,
			vote: Vote,
		) -> DispatchResult {
			let origin = ensure_signed(origin)?;
			ensure!(
				pallet_xcavate_whitelist::Pallet::<T>::whitelisted_accounts(origin.clone()),
				Error::<T>::UserNotWhitelisted
			);
			let current_members = Self::voting_committee();
			ensure!(current_members.contains(&origin), Error::<T>::InsufficientPermission);
			let mut current_vote =
				<OngoingDeletionVotes<T>>::take(proposal_index).ok_or(Error::<T>::InvalidIndex)?;
			let voted = <UserDeletionVotes<T>>::get(proposal_index, origin.clone());
			ensure!(voted.is_none(), Error::<T>::AlreadyVoted);
			if vote == Vote::Yes {
				current_vote.yes_votes += 1;
			} else {
				current_vote.no_votes += 1;
			};

			UserDeletionVotes::<T>::insert(proposal_index, origin.clone(), vote.clone());
			OngoingDeletionVotes::<T>::insert(proposal_index, current_vote);
			Self::deposit_event(Event::<T>::VotedOnDeletion {
				proposal_index,
				member: origin,
				vote,
			});
			Ok(())
		}

		/// Adding a new address to the vote committee.
		///
		/// The origin must be the sudo.
		///
		/// Parameters:
		/// - `member`: The address of the new committee member.
		///
		/// Emits `CommiteeMemberAdded` event when succesfful.
		#[pallet::call_index(9)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::add_committee_member())]
		pub fn add_committee_member(
			origin: OriginFor<T>,
			member: AccountIdOf<T>,
		) -> DispatchResult {
			T::CommitteeOrigin::ensure_origin(origin)?;
			let current_members = Self::voting_committee();
			ensure!(!current_members.contains(&member), Error::<T>::AlreadyMember);
			VotingCommittee::<T>::try_append(member.clone())
				.map_err(|_| Error::<T>::TooManyMembers)?;
			Self::deposit_event(Event::<T>::CommiteeMemberAdded { new_member: member });
			Ok(())
		}
	}

	//** Our helper functions.**//

	impl<T: Config> Pallet<T> {
		/// Get the account id of the pallet.
		pub fn account_id() -> AccountIdOf<T> {
			T::PalletId::get().into_account_truncating()
		}

		/// The needed bond for a proposal whose spend is `value`.
		fn calculate_bond(value: BalanceOf<T>) -> BalanceOf<T> {
			let mut r = T::ProposalBondMinimum::get().max(T::ProposalBond::get() * value);
			if let Some(m) = T::ProposalBondMaximum::get() {
				r = r.min(m);
			}
			r
		}

		/// Rejects the proposal for a loan.
		fn reject_loan_proposal(proposal_index: ProposalIndex) -> DispatchResult {
			let proposal = <Proposals<T>>::take(proposal_index).ok_or(Error::<T>::InvalidIndex)?;
			let value = proposal.bond;
			let imbalance =
				<T as pallet::Config>::Currency::slash_reserved(&proposal.proposer, value).0;
			T::OnSlash::on_unbalanced(imbalance);
			let reserved_value = Self::reserved_loan_amount()
				.checked_sub(Self::balance_to_u64(proposal.amount)?)
				.ok_or(Error::<T>::ArithmeticUnderflow)?;
			ReservedLoanAmount::<T>::put(reserved_value);

			Proposals::<T>::remove(proposal_index);

			Self::deposit_event(Event::<T>::Rejected { proposal_index });
			Ok(())
		}

		/// Approves the proposal and creates the loan.
		fn approve_loan_proposal(proposal_index: ProposalIndex) -> DispatchResult {
			let proposal = <Proposals<T>>::take(proposal_index).ok_or(Error::<T>::InvalidIndex)?;
			let err_amount =
				<T as pallet::Config>::Currency::unreserve(&proposal.proposer, proposal.bond);
			debug_assert!(err_amount.is_zero());
			let user = proposal.beneficiary;
			let value = proposal.amount;
			let mut milestones = proposal.milestones;
			let timestamp = T::TimeProvider::now().as_secs();
			let amount = Self::balance_to_u128(value)?
				.saturating_mul(milestones[0].percentage_to_unlock.deconstruct() as u128)
				.checked_div(100)
				.ok_or(Error::<T>::DivisionError)?;
			milestones.remove(0);
			let available_amount = Self::u128_to_balance_option(amount)?;
			let loan_apy = proposal.apr_rate;
			if pallet_nfts::NextCollectionId::<T>::get().is_none() {
				pallet_nfts::NextCollectionId::<T>::set(
					<T as pallet_nfts::Config>::CollectionId::initial_value(),
				);
			};
			let collection_id =
				pallet_nfts::NextCollectionId::<T>::get().ok_or(Error::<T>::UnknownCollection)?;
			let next_collection_id = collection_id.increment();
			pallet_nfts::NextCollectionId::<T>::set(next_collection_id);
			let collection_id: CollectionId<T> = collection_id.into();
			let item_id: <T as pallet::Config>::ItemId = proposal_index.into();
			let loan_info = LoanInfo {
				borrower: user.clone(),
				loan_amount: value,
				current_loan_balance: value,
				available_amount,
				borrowed_amount: Default::default(),
				charged_interests: Default::default(),
				milestones,
				collection_id,
				item_id,
				loan_apy,
				last_timestamp: timestamp,
				withdraw_lock: Default::default(),
			};

			let loan_index = Self::loan_count() + 1;

			Loans::<T>::insert(loan_index, loan_info);
			OngoingLoans::<T>::try_append(loan_index).map_err(|_| Error::<T>::TooManyLoans)?;
			// calls the create collection function from the uniques pallet, and set the admin as
			// the admin of the collection
			pallet_nfts::Pallet::<T>::do_create_collection(
				collection_id.into(),
				Self::account_id(),
				Self::account_id(),
				Self::default_collection_config(),
				T::CollectionDeposit::get(),
				pallet_nfts::Event::Created {
					creator: Self::account_id(),
					owner: Self::account_id(),
					collection: collection_id.into(),
				},
			)?;
			// calls the mint collection function from the uniques pallet, mints a nft and puts
			// the loan contract as the owner
			pallet_nfts::Pallet::<T>::do_mint(
				collection_id.into(),
				item_id.into(),
				Some(Self::account_id()),
				Self::account_id(),
				Self::default_item_config(),
				|_, _| Ok(()),
			)?;

			let new_value = Self::total_loan_amount() + Self::balance_to_u64(value)?;
			TotalLoanAmount::<T>::put(new_value);
			Proposals::<T>::remove(proposal_index);
			LoanCount::<T>::put(loan_index);
			Self::deposit_event(Event::<T>::Approved { proposal_index });
			Ok(())
		}

		/// Charges the apy and adds the amount to the loan.
		fn charge_apy() -> DispatchResult {
			let ongoing_loans = Self::ongoing_loans();
			for i in ongoing_loans {
				let loan_index = i;
				let mut loan = <Loans<T>>::take(loan_index).ok_or(Error::<T>::InvalidIndex)?;
				let current_timestamp = T::TimeProvider::now().as_secs();
				let time_difference = current_timestamp - loan.last_timestamp;
				let loan_amount =
					Self::balance_to_u64(loan.current_loan_balance + loan.charged_interests)?;
				let interests =
					loan_amount * time_difference * loan.loan_apy / 365 / 60 / 60 / 24 / 10000;
				let interest_balance = Self::u64_to_balance_option(interests)?;
				loan.borrowed_amount += interest_balance;
				loan.charged_interests += interest_balance;
				loan.last_timestamp = current_timestamp;
				Loans::<T>::insert(loan_index, loan.clone());
				let new_value = Self::total_loan_interests() + interests;
				TotalLoanInterests::<T>::put(new_value);
				Self::deposit_event(Event::<T>::ApyCharged { loan_index, interest_balance });
			}
			Ok(())
		}

		/// Add the unlocked milestone when a milestone gets approved.
		fn updating_available_amount(
			loan_id: LoanIndex,
			proposal_index: &ProposalIndex,
		) -> DispatchResult {
			let mut loan = <Loans<T>>::take(loan_id).ok_or(Error::<T>::InvalidIndex)?;
			let loan_amount = loan.loan_amount;
			let mut loan_milestones = loan.milestones;
			let added_available_amount = Self::balance_to_u64(loan_amount)?
				.saturating_mul(loan_milestones[0].percentage_to_unlock.deconstruct() as u64)
				.checked_div(100)
				.ok_or(Error::<T>::DivisionError)?;
			loan_milestones.remove(0);
			let new_available_amount = loan
				.available_amount
				.saturating_add(Self::u64_to_balance_option(added_available_amount)?);
			loan.milestones = loan_milestones;
			loan.available_amount = new_available_amount;
			let proposal_info =
				<MilestoneInfo<T>>::take(proposal_index).ok_or(Error::<T>::InvalidIndex)?;
			let _err_amount = <T as pallet::Config>::Currency::unreserve(
				&proposal_info.proposer,
				proposal_info.bond,
			);
			Loans::<T>::insert(loan_id, loan);
			Self::deposit_event(Event::<T>::MilestoneApproved { loan_id });
			Ok(())
		}

		/// Rejects a milestone proposal.
		fn reject_milestone(proposal_index: &ProposalIndex) -> DispatchResult {
			let proposal_info =
				<MilestoneInfo<T>>::take(proposal_index).ok_or(Error::<T>::InvalidIndex)?;
			let imbalance = <T as pallet::Config>::Currency::slash_reserved(
				&proposal_info.proposer,
				proposal_info.bond,
			)
			.0;
			T::OnSlash::on_unbalanced(imbalance);
			Self::deposit_event(Event::<T>::MilestoneRejected { proposal_index: *proposal_index });
			Ok(())
		}

		/// Deletes a loan when a deletion proposal has been successfull.
		fn delete_loan(loan_id: LoanIndex) -> DispatchResult {
			let loan = <Loans<T>>::take(loan_id).ok_or(Error::<T>::InvalidIndex)?;
			ensure!(loan.borrowed_amount.is_zero(), Error::<T>::LoanStillOngoing);
			let collection_id = loan.collection_id;
			let item_id = loan.item_id;
			pallet_nfts::Pallet::<T>::do_burn(collection_id.into(), item_id.into(), |_| Ok(()))?;
			let mut loans = Self::ongoing_loans();
			let index = loans.iter().position(|x| *x == loan_id).ok_or(Error::<T>::NoLoanFound)?;
			loans.remove(index);
			let reserved_loan = Self::reserved_loan_amount()
				.checked_sub(Self::balance_to_u64(loan.current_loan_balance)?)
				.ok_or(Error::<T>::ArithmeticUnderflow)?;
			ReservedLoanAmount::<T>::put(reserved_loan);
			OngoingLoans::<T>::put(loans);
			Loans::<T>::remove(loan_id);
			Self::deposit_event(Event::<T>::Deleted { loan_index: loan_id });
			Ok(())
		}

		/// Opens the withdraws again when a deletion proposal got rejected.
		fn open_withdrawl(loan_id: LoanIndex) -> DispatchResult {
			let mut loan = <Loans<T>>::take(loan_id).ok_or(Error::<T>::InvalidIndex)?;
			loan.withdraw_lock = false;
			Loans::<T>::insert(loan_id, loan);
			Ok(())
		}

		/// Calculates the apr for a loan.
		fn calculate_apr(experience: u64, loan_term: u64) -> u64 {
			let experinece_number: u32 = match experience {
				1..=5 => 17,
				6..=15 => 15,
				16..=20 => 13,
				_ => 12,
			};

			let loan_term_number: u32 = match loan_term {
				1..=12 => 14,
				13..=24 => 13,
				_ => 12,
			};

			let apr = BASEINTERESTRATE * experinece_number * loan_term_number;
			let interest_points = apr / 100;
			interest_points as u64
		}

		pub fn balance_to_u64(input: BalanceOf<T>) -> Result<u64, Error<T>> {
			TryInto::<u64>::try_into(input).map_err(|_| Error::<T>::ConversionError)
		}

		pub fn balance_to_u128(input: BalanceOf<T>) -> Result<u128, Error<T>> {
			TryInto::<u128>::try_into(input).map_err(|_| Error::<T>::ConversionError)
		}

		pub fn u64_to_balance_option(input: u64) -> Result<BalanceOf<T>, Error<T>> {
			input.try_into().map_err(|_| Error::<T>::ConversionError)
		}

		pub fn u128_to_balance_option(input: u128) -> Result<BalanceOf<T>, Error<T>> {
			input.try_into().map_err(|_| Error::<T>::ConversionError)
		}

		fn default_collection_config() -> CollectionConfig<
			BalanceOf1<T>,
			BlockNumberFor<T>,
			<T as pallet_nfts::Config>::CollectionId,
		> {
			Self::collection_config_from_disabled_settings(
				CollectionSetting::DepositRequired.into(),
			)
		}

		fn collection_config_from_disabled_settings(
			settings: BitFlags<CollectionSetting>,
		) -> CollectionConfig<
			BalanceOf1<T>,
			BlockNumberFor<T>,
			<T as pallet_nfts::Config>::CollectionId,
		> {
			CollectionConfig {
				settings: CollectionSettings::from_disabled(settings),
				max_supply: None,
				mint_settings: MintSettings::default(),
			}
		}

		fn default_item_config() -> ItemConfig {
			ItemConfig { settings: ItemSettings::all_enabled() }
		}
	}
}