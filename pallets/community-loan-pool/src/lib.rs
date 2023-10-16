#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
/// <https://docs.substrate.io/reference/frame-pallets/>
/// <https://docs.substrate.io/reference/frame-pallets/>
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
	sp_runtime,
	traits::{
		Currency, ExistenceRequirement::KeepAlive, Get, OnUnbalanced, ReservableCurrency, UnixTime,
	},
	PalletId,
};

pub use pallet_nfts::{
	CollectionConfig, CollectionSetting, CollectionSettings, ItemConfig, ItemSettings, MintSettings,
};

use sp_std::prelude::*;

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

type BalanceOf1<T> = <<T as pallet_nfts::Config>::Currency as Currency<
	<T as frame_system::Config>::AccountId,
>>::Balance;

pub type BoundedProposedMilestones<T> =
	BoundedVec<ProposedMilestone, <T as Config>::MaxMilestonesPerProject>;

pub const BASEINTERESTRATE: f32 = 5.25;

#[cfg(feature = "runtime-benchmarks")]
pub struct NftHelper;

#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[derive(Encode, Decode, Clone, PartialEq, Eq, MaxEncodedLen, RuntimeDebug, TypeInfo)]
pub struct ProposedMilestone {
	pub percentage_to_unlock: Percent,
}

#[cfg(feature = "runtime-benchmarks")]
pub trait BenchmarkHelper<CollectionId, ItemId> {
	pub fn to_collection(i: u32) -> CollectionId;
	pub fn to_nft(i: u32) -> ItemId;
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
	use frame_support::sp_runtime::{SaturatedConversion, Saturating};
	use frame_system::pallet_prelude::*;
	use scale_info::TypeInfo;

	pub type ProposalIndex = u32;
	pub type LoanIndex = u32;

	/// A loan proposal
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

	#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
	#[derive(Encode, Decode, Clone, PartialEq, Eq, MaxEncodedLen, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct MilestoneProposalInfo<Balance, T: Config> {
		proposer: AccountIdOf<T>,
		bond: Balance,
	}

	/// loan info
	#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
	#[derive(Encode, Decode, Clone, PartialEq, Eq, MaxEncodedLen, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct LoanInfo<Balance, CollectionId, ItemId, T: Config> {
		pub borrower: AccountIdOf<T>,
		pub loan_amount: Balance,
		pub current_loan_balance: Balance,
		pub available_amount: Balance,
		pub borrowed_amount: Balance,
		pub milestones: BoundedProposedMilestones<T>,
		pub collection_id: CollectionId,
		pub item_id: ItemId,
		pub loan_apy: LoanApy,
		pub last_timestamp: u64,
		pub withdraw_lock: bool,
	}

	/// AccountId storage
	#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
	#[derive(Encode, Decode, Clone, PartialEq, Eq, MaxEncodedLen, RuntimeDebug, TypeInfo)]
	pub struct PalletIdStorage<T: Config> {
		pallet_id: AccountIdOf<T>,
	}

	#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
	#[derive(Encode, Decode, Clone, PartialEq, Eq, MaxEncodedLen, RuntimeDebug, TypeInfo)]
	pub enum Vote {
		Yes,
		No,
	}

	#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
	#[derive(Encode, Decode, Clone, PartialEq, Eq, MaxEncodedLen, RuntimeDebug, TypeInfo)]
	pub struct VoteStats {
		pub yes_votes: u64,
		pub no_votes: u64,
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_nfts::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// The currency type.
		type Currency: Currency<AccountIdOf<Self>> + ReservableCurrency<AccountIdOf<Self>>;

		/// Origin from which rejections must come.
		type RejectOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		/// Origin from which approves must come.
		type ApproveOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		/// Origin who can add or remove committee members
		type CommitteeOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		/// Origin who can delete loans
		type DeleteOrigin: EnsureOrigin<Self::RuntimeOrigin>;

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

		/// The treasury's pallet id, used for deriving its sovereign account ID.
		#[pallet::constant]
		type PalletId: Get<PalletId>;

		/// Handler for the unbalanced decrease when slashing for a rejected proposal or bounty.
		type OnSlash: OnUnbalanced<NegativeImbalanceOf<Self>>;

		/// The maximum amount of loans that can run at the same time.
		#[pallet::constant]
		type MaxOngoingLoans: Get<u32>;

		/// lose coupling of pallet timestamp
		type TimeProvider: UnixTime;

		/// Type representing the weight of this pallet
		type WeightInfo: WeightInfo;

		#[cfg(feature = "runtime-benchmarks")]
		type Helper: crate::BenchmarkHelper<Self::CollectionId, Self::ItemId>;

		/// The amount of time given to vote for a proposal
		type VotingTime: Get<BlockNumberFor<Self>>;

		/// The maximum amount of commitee members
		type MaxCommitteeMembers: Get<u32>;

		/// The maximum of milestones for a lone
		type MaxMilestonesPerProject: Get<u32>;
	}

	/// Vec of admins who are able to vote
	#[pallet::storage]
	#[pallet::getter(fn voting_committee)]
	pub(super) type VotingCommittee<T: Config> =
		StorageValue<_, BoundedVec<AccountIdOf<T>, T::MaxOngoingLoans>, ValueQuery>;

	/// Number of proposals that have been made.
	#[pallet::storage]
	#[pallet::getter(fn loan_count)]
	pub(super) type LoanCount<T> = StorageValue<_, ProposalIndex, ValueQuery>;

	/// Number of proposals that have been made.
	#[pallet::storage]
	#[pallet::getter(fn proposal_count)]
	pub(super) type ProposalCount<T> = StorageValue<_, ProposalIndex, ValueQuery>;

	/// Number of milestone proposal that have been made.
	#[pallet::storage]
	#[pallet::getter(fn milestone_proposal_count)]
	pub(super) type MilestoneProposalCount<T> = StorageValue<_, ProposalIndex, ValueQuery>;

	/// Number of deletion proposal that have been made
	#[pallet::storage]
	#[pallet::getter(fn deletion_proposal_count)]
	pub(super) type DeletionProposalCount<T> = StorageValue<_, ProposalIndex, ValueQuery>;

	/// Total amount of loan funds
	#[pallet::storage]
	#[pallet::getter(fn total_loan_amount)]
	pub(super) type TotalLoanAmount<T> = StorageValue<_, u64, ValueQuery>;

	/// Amount of founds that is still on the pallet but is reserved for loan
	#[pallet::storage]
	#[pallet::getter(fn reserved_loan_amount)]
	pub(super) type ReservedLoanAmount<T> = StorageValue<_, u64, ValueQuery>;

	/// All currently ongoing loans
	#[pallet::storage]
	#[pallet::getter(fn ongoing_loans)]
	pub(super) type OngoingLoans<T: Config> =
		StorageValue<_, BoundedVec<ProposalIndex, T::MaxOngoingLoans>, ValueQuery>;

	/// Proposals that have been made.
	#[pallet::storage]
	#[pallet::getter(fn proposals)]
	pub(super) type Proposals<T> = StorageMap<
		_,
		Twox64Concat,
		ProposalIndex,
		Proposal<BalanceOf<T>, BlockNumberFor<T>, T>,
		OptionQuery,
	>;

	/// Milestone proposal that has been made.
	#[pallet::storage]
	#[pallet::getter(fn milestone_proposals)]
	pub(super) type MilestoneProposals<T> =
		StorageMap<_, Twox64Concat, ProposalIndex, LoanIndex, OptionQuery>;

	/// Milestone proposal that has been made.
	#[pallet::storage]
	#[pallet::getter(fn milestone_bond)]
	pub(super) type MilestoneBond<T> = StorageMap<
		_,
		Twox64Concat,
		ProposalIndex,
		MilestoneProposalInfo<BalanceOf<T>, T>,
		OptionQuery,
	>;

	/// Deletion proposal that has been made.
	#[pallet::storage]
	#[pallet::getter(fn deletion_proposals)]
	pub(super) type DeletionProposals<T> =
		StorageMap<_, Twox64Concat, ProposalIndex, LoanIndex, OptionQuery>;

	/// Mapping of ongoing loans
	#[pallet::storage]
	#[pallet::getter(fn loans)]
	pub(super) type Loans<T: Config> = StorageMap<
		_,
		Twox64Concat,
		LoanIndex,
		LoanInfo<BalanceOf<T>, T::CollectionId, T::ItemId, T>,
		OptionQuery,
	>;

	/// Mapping of ongoing votes
	#[pallet::storage]
	#[pallet::getter(fn ongoing_votes)]
	pub(super) type OngoingVotes<T: Config> =
		StorageMap<_, Twox64Concat, ProposalIndex, VoteStats, OptionQuery>;

	/// Mapping of ongoing votes
	#[pallet::storage]
	#[pallet::getter(fn ongoing_milestone_votes)]
	pub(super) type OngoingMilestoneVotes<T: Config> =
		StorageMap<_, Twox64Concat, ProposalIndex, VoteStats, OptionQuery>;

	/// Mapping of ongoing deletion votes
	#[pallet::storage]
	#[pallet::getter(fn ongoing_deletion_votes)]
	pub(super) type OngoingDeletionVotes<T: Config> =
		StorageMap<_, Twox64Concat, ProposalIndex, VoteStats, OptionQuery>;

	/// Mapping of user who voted for a proposal
	#[pallet::storage]
	#[pallet::getter(fn user_milestone_votes)]
	pub(super) type UserMilestoneVotes<T: Config> =
		StorageMap<_, Twox64Concat, (ProposalIndex, AccountIdOf<T>), Vote, OptionQuery>;

	/// Mapping of user who voted for a milestone proposal
	#[pallet::storage]
	#[pallet::getter(fn user_votes)]
	pub(super) type UserVotes<T: Config> =
		StorageMap<_, Twox64Concat, (ProposalIndex, AccountIdOf<T>), Vote, OptionQuery>;

	/// Mapping of user who voted for a deletion proposal
	#[pallet::storage]
	#[pallet::getter(fn user_deletion_votes)]
	pub(super) type UserDeletionVotes<T: Config> =
		StorageMap<_, Twox64Concat, (ProposalIndex, AccountIdOf<T>), Vote, OptionQuery>;

	/// Stores the project keys and round types ending on a given block
	#[pallet::storage]
	pub type RoundsExpiring<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BlockNumberFor<T>,
		BoundedVec<ProposalIndex, T::MaxOngoingLoans>,
		ValueQuery,
	>;

	/// Stores the project keys and round types ending on a given block
	#[pallet::storage]
	pub type MilestoneRoundsExpiring<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BlockNumberFor<T>,
		BoundedVec<ProposalIndex, T::MaxOngoingLoans>,
		ValueQuery,
	>;

	/// Stores the project keys and round types ending on a given block
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
			// Create Treasury account
			let account_id = <Pallet<T>>::account_id();
			let min = <T as pallet::Config>::Currency::minimum_balance();
			if <T as pallet::Config>::Currency::free_balance(&account_id) < min {
				let _ = <T as pallet::Config>::Currency::make_free_balance_be(&account_id, min);
			}
		}
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Proposer's balance is too low
		InsufficientProposersBalance,
		/// Loan pool's balance is too low
		InsufficientLoanPoolBalance,
		/// No proposal index
		InvalidIndex,
		/// The caller doesn't have enough permission
		InsufficientPermission,
		/// Max amount of ongoing loan reached
		TooManyLoans,
		/// User has already voted
		AlreadyVoted,
		/// Loan got not approved
		NotApproved,
		/// The account is already a member in the voting committee
		AlreadyMember,
		/// There are already enough committee members
		TooManyMembers,
		/// There are not enough funds available in the loan
		NotEnoughFundsToWithdraw,
		/// The loan is still ongoing
		LoanStillOngoing,
		/// All milestones has been accomplished
		NoMilestonesLeft,
		/// Milestones of the loan have to be 100 % in Sum
		MilestonesHaveToCoverLoan,
		/// Withdrawl is locked during ongoing voting for deletion
		DeletionVotingOngoing,
		/// The beneficiary didn't borrow that much funds
		WantsToRepayTooMuch,
		/// There are not enough funds available in the loan pallet
		NotEnoughLoanFundsAvailable,
		/// The Milestones for the proposal have already been set
		MilestonesAlreadySet,
		/// There has been no milestones set in the proposal
		NoMilestones,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// New Proposal
		Proposed { proposal_index: ProposalIndex },
		/// New Milestone Proposal
		MilestoneProposed { proposal_index: ProposalIndex },
		/// New Deletion Proposal
		DeletionProposed { proposal_index: ProposalIndex, loan_index: LoanIndex },
		/// Proposal has been approved
		Approved { proposal_index: ProposalIndex },
		/// Proposal has been rejected
		Rejected { proposal_index: ProposalIndex },
		/// Loan has been deleted
		Deleted { loan_index: LoanIndex },
		/// Charged APY
		ApyCharged { loan_index: LoanIndex },
		/// Loan has been updated
		LoanUpdated { loan_index: LoanIndex },
		/// User withdraw money
		Withdraw { loan_index: LoanIndex, amount: BalanceOf<T> },
		/// Voted on a proposal
		VotedOnProposal { proposal_index: ProposalIndex, member: AccountIdOf<T>, vote: Vote },
		/// Voted on a milestone
		VotedOnMilestone { proposal_index: ProposalIndex, member: AccountIdOf<T>, vote: Vote },
		/// Voted on a deletion
		VotedOnDeletion { proposal_index: ProposalIndex, member: AccountIdOf<T>, vote: Vote },
		/// A new committee member has been added
		CommiteeMemberAdded { new_member: AccountIdOf<T> },
		/// Milestone Proposal has been approved
		MilestoneApproved { loan_id: LoanIndex },
		/// Milestone Proposal has been rejected
		MilestoneRejected { proposal_index: ProposalIndex },
		/// Milestones have been set for a proposal
		MilestonesSet { proposal_index: ProposalIndex },
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T>
	where
		<T as pallet_nfts::Config>::CollectionId: From<u32>,
		<T as pallet_nfts::Config>::ItemId: From<u32>,
	{
		fn on_initialize(n: frame_system::pallet_prelude::BlockNumberFor<T>) -> Weight {
			let mut weight = T::DbWeight::get().reads_writes(1, 1);

			let ended_votings = RoundsExpiring::<T>::take(n);

			ended_votings.iter().for_each(|item| {
				weight = weight.saturating_add(T::DbWeight::get().reads_writes(1, 1));
				let voting_result = <OngoingVotes<T>>::take(item);
				if let Some(voting_result) = voting_result {
					if voting_result.yes_votes > voting_result.no_votes {
						Self::approve_loan_proposal(*item).unwrap_or_default();
					} else {
						Self::reject_loan_proposal(*item).unwrap_or_default();
					}
					OngoingVotes::<T>::remove(item);
				}
			});

			let ended_milestone_votes = MilestoneRoundsExpiring::<T>::take(n);

			ended_milestone_votes.iter().for_each(|item| {
				weight = weight.saturating_add(T::DbWeight::get().reads_writes(1, 1));
				let voting_result = <OngoingMilestoneVotes<T>>::take(item);
				if let Some(voting_result) = voting_result {
					if voting_result.yes_votes > voting_result.no_votes {
						let loan_id = <MilestoneProposals<T>>::take(item);
						if let Some(loan_id) = loan_id {
							Self::updating_available_amount(loan_id, item);
						}
					} else {
						Self::reject_milestone(item);
					}
					OngoingMilestoneVotes::<T>::remove(item);
				}
			});

			let ended_deletion_votes = DeletionRoundsExpiring::<T>::take(n);

			ended_deletion_votes.iter().for_each(|item| {
				weight = weight.saturating_add(T::DbWeight::get().reads_writes(1, 1));
				let voting_result = <OngoingDeletionVotes<T>>::take(item);
				let loan_id = <DeletionProposals<T>>::take(item);
				if let Some(voting_result) = voting_result {
					if voting_result.yes_votes > voting_result.no_votes {
						if let Some(loan_id) = loan_id {
							Self::delete_loan(loan_id);
						}
					} else if let Some(loan_id) = loan_id {
						Self::open_withdrawl(loan_id);
					}

					OngoingDeletionVotes::<T>::remove(item);
				}
			});

			weight
		}

		fn on_finalize(_n: frame_system::pallet_prelude::BlockNumberFor<T>) {
			//let block = n.saturated_into::<u64>();
			//if block % 10 == 0 {
			Self::charge_apy().unwrap_or_default();
			//}
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Apply for a loan. A deposit amount is reserved
		/// and slashed if the proposal is rejected. It is returned once the proposal is awarded.
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
			let beneficiary = T::Lookup::lookup(beneficiary)?;
			let total_loan_amount =
				Self::u64_to_balance_option(Self::reserved_loan_amount()).unwrap();
			//let decimal = 1000000000000_u64.saturated_into();
			ensure!(
				<T as pallet::Config>::Currency::free_balance(&Self::account_id())
					>= total_loan_amount.saturating_add(amount),
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
			OngoingVotes::<T>::insert(proposal_index, vote_stats);
			Proposals::<T>::insert(proposal_index, proposal);
			ProposalCount::<T>::put(proposal_index);

			Self::deposit_event(Event::Proposed { proposal_index });
			Ok(())
		}

		/// Applying for the next milestone in the ongoing loan.
		#[pallet::call_index(1)]
		#[pallet::weight(0)]
		pub fn propose_milestone(origin: OriginFor<T>, loan_id: LoanIndex) -> DispatchResult {
			let origin = ensure_signed(origin)?;
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
			MilestoneBond::<T>::insert(milestone_proposal_index, milestone_details);
			MilestoneProposals::<T>::insert(milestone_proposal_index, loan_id);
			MilestoneProposalCount::<T>::put(milestone_proposal_index);
			Self::deposit_event(Event::MilestoneProposed {
				proposal_index: milestone_proposal_index,
			});
			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(0)]
		pub fn propose_deletion(origin: OriginFor<T>, loan_id: LoanIndex) -> DispatchResult {
			let origin = ensure_signed(origin.clone())?;
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

		/// Withdraw an certain amount of the loan from the pallet.
		///
		/// May only be called from the loan borrower.

		#[pallet::call_index(3)]
		#[pallet::weight(0)]
		pub fn withdraw(
			origin: OriginFor<T>,
			loan_id: LoanIndex,
			amount: BalanceOf<T>,
		) -> DispatchResult {
			let signer = ensure_signed(origin.clone())?;
			let mut loan = <Loans<T>>::take(loan_id).ok_or(Error::<T>::InvalidIndex)?;
			ensure!(signer == loan.borrower, Error::<T>::InsufficientPermission);
			ensure!(amount <= loan.available_amount, Error::<T>::NotEnoughFundsToWithdraw);
			ensure!(!loan.withdraw_lock, Error::<T>::DeletionVotingOngoing);
			let loan_pallet = Self::account_id();
			let sending_amount = Self::balance_to_u64(amount).unwrap();
			// For unit tests the line with the
			<T as pallet::Config>::Currency::transfer(
				&loan_pallet,
				&signer,
				// For unit tests this line has to be commented out and the line blow has to be uncommented due to the dicmals on polkadot js
				(sending_amount as u128 * 1000000000000).try_into().ok().unwrap_or_default(),
				//amount,
				KeepAlive,
			)
			.unwrap_or_default();
			loan.borrowed_amount = loan.borrowed_amount.saturating_add(amount);
			loan.available_amount = loan.available_amount.saturating_sub(amount);
			Loans::<T>::insert(loan_id, loan);
			let reserved_value =
				Self::reserved_loan_amount() - Self::balance_to_u64(amount).unwrap();
			ReservedLoanAmount::<T>::put(reserved_value);
			Self::deposit_event(Event::<T>::Withdraw { loan_index: loan_id, amount });
			Ok(())
		}

		/// Repays a certain amount of the loan from the pallet.
		///
		/// May only be called from the loan borrower.

		#[pallet::call_index(4)]
		#[pallet::weight(0)]
		pub fn repay(
			origin: OriginFor<T>,
			loan_id: LoanIndex,
			amount: BalanceOf<T>,
		) -> DispatchResult {
			let signer = ensure_signed(origin.clone())?;
			let mut loan = <Loans<T>>::take(loan_id).ok_or(Error::<T>::InvalidIndex)?;
			ensure!(amount <= loan.borrowed_amount, Error::<T>::WantsToRepayTooMuch);
			//ensure!(signer == loan.contract_account_id, Error::<T>::InsufficientPermission);
			let loan_pallet = Self::account_id();
			let sending_amount = Self::balance_to_u64(amount).unwrap();
			<T as pallet::Config>::Currency::transfer(
				&signer,
				&loan_pallet,
				// For unit tests this line has to be commented out and the line blow has to be uncommented due to the dicmals on polkadot js
				(sending_amount * 1000000000000).try_into().ok().unwrap(),
				//amount,
				KeepAlive,
			)
			.unwrap_or_default();
			loan.borrowed_amount = loan.borrowed_amount.saturating_sub(amount);
			loan.current_loan_balance = loan.current_loan_balance.saturating_sub(amount);
			Loans::<T>::insert(loan_id, loan);
			let new_value = Self::total_loan_amount() - Self::balance_to_u64(amount).unwrap();
			TotalLoanAmount::<T>::put(new_value);
			Self::deposit_event(Event::<T>::LoanUpdated { loan_index: loan_id });
			Ok(())
		}

		/// The committee sets the milestone distribution for the loan

		#[pallet::call_index(55)]
		#[pallet::weight(0)]
		pub fn set_milestones(
			origin: OriginFor<T>,
			proposal_index: ProposalIndex,
			proposed_milestones: BoundedProposedMilestones<T>,
		) -> DispatchResult {
			let origin = ensure_signed(origin)?;
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
			let voted = <UserVotes<T>>::get((proposal_index, origin.clone()));
			if voted.is_none() {
				current_vote.yes_votes += 1;
				UserVotes::<T>::insert((proposal_index, origin.clone()), Vote::Yes);
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

		/// Let committee members vote for a proposal
		#[pallet::call_index(5)]
		#[pallet::weight(0)]
		pub fn vote_on_proposal(
			origin: OriginFor<T>,
			proposal_index: ProposalIndex,
			vote: Vote,
		) -> DispatchResult {
			let origin = ensure_signed(origin)?;
			let current_members = Self::voting_committee();
			ensure!(current_members.contains(&origin), Error::<T>::InsufficientPermission);
			let mut current_vote =
				<OngoingVotes<T>>::take(proposal_index).ok_or(Error::<T>::InvalidIndex)?;
			let voted = <UserVotes<T>>::get((proposal_index, origin.clone()));
			ensure!(voted.is_none(), Error::<T>::AlreadyVoted);
			let proposal = Self::proposals(proposal_index).unwrap_or_default();
			ensure!(proposal.milestones.len() > 0, Error::<T>::NoMilestones);
			if vote == Vote::Yes {
				current_vote.yes_votes += 1;
			} else {
				current_vote.no_votes += 1;
			};

			UserVotes::<T>::insert((proposal_index, origin.clone()), vote.clone());
			OngoingVotes::<T>::insert(proposal_index, current_vote);
			Self::deposit_event(Event::<T>::VotedOnProposal {
				proposal_index,
				member: origin,
				vote,
			});
			Ok(())
		}

		/// Let committee vote on milestone proposal
		#[pallet::call_index(6)]
		#[pallet::weight(0)]
		pub fn vote_on_milestone_proposal(
			origin: OriginFor<T>,
			proposal_index: ProposalIndex,
			vote: Vote,
		) -> DispatchResult {
			let origin = ensure_signed(origin)?;
			let current_members = Self::voting_committee();
			ensure!(current_members.contains(&origin), Error::<T>::InsufficientPermission);
			let mut current_vote =
				<OngoingMilestoneVotes<T>>::take(proposal_index).ok_or(Error::<T>::InvalidIndex)?;
			let voted = <UserMilestoneVotes<T>>::get((proposal_index, origin.clone()));
			ensure!(voted.is_none(), Error::<T>::AlreadyVoted);
			if vote == Vote::Yes {
				current_vote.yes_votes += 1;
			} else {
				current_vote.no_votes += 1;
			};

			UserMilestoneVotes::<T>::insert((proposal_index, origin.clone()), vote.clone());
			OngoingMilestoneVotes::<T>::insert(proposal_index, current_vote);
			Self::deposit_event(Event::<T>::VotedOnMilestone {
				proposal_index,
				member: origin,
				vote,
			});
			Ok(())
		}

		/// Let committee vote on deletion proposal
		#[pallet::call_index(7)]
		#[pallet::weight(0)]
		pub fn vote_on_deletion_proposal(
			origin: OriginFor<T>,
			proposal_index: ProposalIndex,
			vote: Vote,
		) -> DispatchResult {
			let origin = ensure_signed(origin)?;
			let current_members = Self::voting_committee();
			ensure!(current_members.contains(&origin), Error::<T>::InsufficientPermission);
			let mut current_vote =
				<OngoingDeletionVotes<T>>::take(proposal_index).ok_or(Error::<T>::InvalidIndex)?;
			let voted = <UserDeletionVotes<T>>::get((proposal_index, origin.clone()));
			ensure!(voted.is_none(), Error::<T>::AlreadyVoted);
			if vote == Vote::Yes {
				current_vote.yes_votes += 1;
			} else {
				current_vote.no_votes += 1;
			};

			UserDeletionVotes::<T>::insert((proposal_index, origin.clone()), vote.clone());
			OngoingDeletionVotes::<T>::insert(proposal_index, current_vote);
			Self::deposit_event(Event::<T>::VotedOnDeletion {
				proposal_index,
				member: origin,
				vote,
			});
			Ok(())
		}

		/// Adding a new address to the vote committee
		#[pallet::call_index(8)]
		#[pallet::weight(0)]
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
		pub fn account_id() -> AccountIdOf<T> {
			T::PalletId::get().into_account_truncating()
		}

		fn calculate_bond(value: BalanceOf<T>) -> BalanceOf<T> {
			let mut r = T::ProposalBondMinimum::get().max(T::ProposalBond::get() * value);
			if let Some(m) = T::ProposalBondMaximum::get() {
				r = r.min(m);
			}
			r
		}

		fn reject_loan_proposal(proposal_index: ProposalIndex) -> DispatchResult {
			let proposal = <Proposals<T>>::take(proposal_index).ok_or(Error::<T>::InvalidIndex)?;
			let value = proposal.bond;
			let imbalance =
				<T as pallet::Config>::Currency::slash_reserved(&proposal.proposer, value).0;
			T::OnSlash::on_unbalanced(imbalance);

			Proposals::<T>::remove(proposal_index);

			Self::deposit_event(Event::<T>::Rejected { proposal_index });
			Ok(())
		}

		fn approve_loan_proposal(proposal_index: ProposalIndex) -> DispatchResult
		where
			<T as pallet_nfts::Config>::CollectionId: From<u32>,
			<T as pallet_nfts::Config>::ItemId: From<u32>,
		{
			let proposal = <Proposals<T>>::take(proposal_index).ok_or(Error::<T>::InvalidIndex)?;
			let total_loan_amount = Self::u64_to_balance_option(Self::total_loan_amount()).unwrap();
			//let decimal = 1000000000000_u64.saturated_into();
			//ensure!(<T as pallet::Config>::Currency::free_balance(&Self::account_id()) / decimal >= total_loan_amount.saturating_add(proposal.amount), Error::<T>::NotEnoughLoanFundsAvailable);
			let err_amount =
				<T as pallet::Config>::Currency::unreserve(&proposal.proposer, proposal.bond);
			debug_assert!(err_amount.is_zero());
			let user = proposal.beneficiary;
			let value = proposal.amount;
			let mut milestones = proposal.milestones;
			let timestamp = T::TimeProvider::now().as_secs();
			let amount = Self::balance_to_u64(value).unwrap()
				* milestones[0].percentage_to_unlock.deconstruct() as u64
				/ 100;
			milestones.remove(0);
			let available_amount = Self::u64_to_balance_option(amount).unwrap();
			let loan_apy = proposal.apr_rate;
			let collection_id: T::CollectionId = proposal_index.into();
			let item_id: T::ItemId = proposal_index.into();
			let loan_info = LoanInfo {
				borrower: user.clone(),
				loan_amount: value,
				current_loan_balance: value,
				available_amount,
				borrowed_amount: Default::default(),
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
				collection_id,
				Self::account_id(),
				Self::account_id(),
				Self::default_collection_config(),
				T::CollectionDeposit::get(),
				pallet_nfts::Event::Created {
					creator: Self::account_id(),
					owner: Self::account_id(),
					collection: collection_id,
				},
			)?;
			// calls the mint collection function from the uniques pallet, mints a nft and puts
			// the loan contract as the owner
			pallet_nfts::Pallet::<T>::do_mint(
				collection_id,
				item_id,
				Some(Self::account_id()),
				Self::account_id(),
				Self::default_item_config(),
				|_, _| Ok(()),
			)?;

			let new_value = Self::total_loan_amount() + Self::balance_to_u64(value).unwrap();
			TotalLoanAmount::<T>::put(new_value);
			let reserved_value =
				Self::reserved_loan_amount() + Self::balance_to_u64(value).unwrap();
			ReservedLoanAmount::<T>::put(reserved_value);
			Proposals::<T>::remove(proposal_index);
			LoanCount::<T>::put(loan_index);
			Self::deposit_event(Event::<T>::Approved { proposal_index });
			Ok(())
		}

		// Work in progress, to be implmented in the future
		fn charge_apy() -> DispatchResult {
			let ongoing_loans = Self::ongoing_loans();
			for i in ongoing_loans {
				let loan_index = i;
				let mut loan = <Loans<T>>::take(loan_index).ok_or(Error::<T>::InvalidIndex)?;
				let current_timestamp = T::TimeProvider::now().as_secs();
				let time_difference = current_timestamp - loan.last_timestamp;
				let loan_amount = Self::balance_to_u64(loan.current_loan_balance).unwrap();
				let interests =
					loan_amount * time_difference * loan.loan_apy / 365 / 60 / 60 / 24 / 100 / 100;
				let interest_balance = Self::u64_to_balance_option(interests).unwrap();
				loan.borrowed_amount += interest_balance;
				loan.current_loan_balance += interest_balance;
				loan.last_timestamp = current_timestamp;
				Loans::<T>::insert(loan_index, loan.clone());
				let new_value = Self::total_loan_amount() + interests;
				TotalLoanAmount::<T>::put(new_value);
				Self::deposit_event(Event::<T>::ApyCharged { loan_index });
			}
			Ok(())
		}

		fn updating_available_amount(
			loan_id: LoanIndex,
			proposal_index: &ProposalIndex,
		) -> DispatchResult {
			let mut loan = <Loans<T>>::take(loan_id).ok_or(Error::<T>::InvalidIndex)?;
			let loan_amount = loan.loan_amount;
			let mut loan_milestones = loan.milestones;
			let added_available_amount = Self::balance_to_u64(loan_amount).unwrap()
				* loan_milestones[0].percentage_to_unlock.deconstruct() as u64
				/ 100;
			loan_milestones.remove(0);
			let new_available_amount = loan
				.available_amount
				.saturating_add(Self::u64_to_balance_option(added_available_amount).unwrap());
			loan.milestones = loan_milestones;
			loan.available_amount = new_available_amount;
			let proposal_info = <MilestoneBond<T>>::take(proposal_index)
				.ok_or(Error::<T>::InvalidIndex)
				.unwrap();
			let err_amount = <T as pallet::Config>::Currency::unreserve(
				&proposal_info.proposer,
				proposal_info.bond,
			);
			Loans::<T>::insert(loan_id, loan);
			Self::deposit_event(Event::<T>::MilestoneApproved { loan_id });
			Ok(())
		}

		fn reject_milestone(proposal_index: &ProposalIndex) -> DispatchResult {
			let proposal_info = <MilestoneBond<T>>::take(proposal_index)
				.ok_or(Error::<T>::InvalidIndex)
				.unwrap();
			let imbalance = <T as pallet::Config>::Currency::slash_reserved(
				&proposal_info.proposer,
				proposal_info.bond,
			)
			.0;
			T::OnSlash::on_unbalanced(imbalance);
			Self::deposit_event(Event::<T>::MilestoneRejected { proposal_index: *proposal_index });
			Ok(())
		}

		fn delete_loan(loan_id: LoanIndex) -> DispatchResult {
			let loan = <Loans<T>>::take(loan_id).ok_or(Error::<T>::InvalidIndex)?;
			ensure!(loan.borrowed_amount.is_zero(), Error::<T>::LoanStillOngoing);
			let collection_id = loan.collection_id;
			let item_id = loan.item_id;
			pallet_nfts::Pallet::<T>::do_burn(collection_id, item_id, |_| Ok(()))?;
			let mut loans = Self::ongoing_loans();
			let index = loans.iter().position(|x| *x == loan_id).unwrap();
			loans.remove(index);
			let reserved_loan = Self::reserved_loan_amount()
				- Self::balance_to_u64(loan.current_loan_balance).unwrap();
			ReservedLoanAmount::<T>::put(reserved_loan);
			OngoingLoans::<T>::put(loans);
			Loans::<T>::remove(loan_id);
			Self::deposit_event(Event::<T>::Deleted { loan_index: loan_id });
			Ok(())
		}

		fn open_withdrawl(loan_id: LoanIndex) -> DispatchResult {
			let mut loan = <Loans<T>>::take(loan_id).ok_or(Error::<T>::InvalidIndex)?;
			loan.withdraw_lock = false;
			Loans::<T>::insert(loan_id, loan);
			Ok(())
		}

		fn calculate_apr(experience: u64, loan_term: u64) -> u64 {
			let experinece_number: f32 = match experience {
				1..=5 => 1.7,
				6..=15 => 1.5,
				16..=20 => 1.3,
				_ => 1.2,
			};

			let loan_term_number: f32 = match loan_term {
				1..=12 => 1.4,
				13..=24 => 1.3,
				_ => 1.2,
			};

			let apr = BASEINTERESTRATE * experinece_number * loan_term_number;
			let interest_points = apr * 100.0;
			interest_points as u64
		}

		pub fn balance_to_u64(input: BalanceOf<T>) -> Option<u64> {
			TryInto::<u64>::try_into(input).ok()
		}

		pub fn u64_to_balance_option(input: u64) -> Option<BalanceOf<T>> {
			input.try_into().ok()
		}

		fn default_collection_config(
		) -> CollectionConfig<BalanceOf1<T>, BlockNumberFor<T>, T::CollectionId> {
			Self::collection_config_from_disabled_settings(
				CollectionSetting::DepositRequired.into(),
			)
		}

		fn collection_config_from_disabled_settings(
			settings: BitFlags<CollectionSetting>,
		) -> CollectionConfig<BalanceOf1<T>, BlockNumberFor<T>, T::CollectionId> {
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
