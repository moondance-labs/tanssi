#![cfg_attr(not(feature = "std"), no_std)]
use cumulus_pallet_parachain_system::RelayChainStateProof;
use cumulus_primitives_core::relay_chain::BlakeTwo256;
use cumulus_primitives_core::relay_chain::BlockNumber;
use cumulus_primitives_core::relay_chain::HeadData;
use cumulus_primitives_core::ParaId;
use sp_consensus_aura::inherents::InherentType;
use sp_consensus_aura::AURA_ENGINE_ID;
use sp_inherents::InherentIdentifier;
use sp_runtime::traits::Header;
use sp_runtime::DispatchResult;
use sp_std::prelude::*;
use tp_author_noting_inherent::INHERENT_IDENTIFIER;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod test;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::{DispatchResult, *};
    use frame_support::dispatch::PostDispatchInfo;
    use frame_support::pallet_prelude::*;
    use frame_support::Hashable;
    use frame_system::pallet_prelude::*;
    use tp_author_noting_inherent::PARAS_HEADS_INDEX;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Our own para
        type SelfParaId: Get<ParaId>;

        type AuthorFetcher: GetAuthorFromSlot<Self>;
    }

    pub trait GetAuthorFromSlot<T: Config> {
        /// Returns current session index.
        fn author_from_inherent(inherent: InherentType) -> Option<T::AccountId>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(1)]
        #[pallet::weight((0, DispatchClass::Mandatory))]
        // TODO: This weight should be corrected.
        pub fn set_latest_author_data(
            origin: OriginFor<T>,
            data: tp_author_noting_inherent::OwnParachainInherentData,
        ) -> DispatchResultWithPostInfo {
            let total_weight = Weight::zero();
            ensure_none(origin)?;
            let tp_author_noting_inherent::OwnParachainInherentData {
                validation_data: vfp,
                relay_chain_state,
            } = data;

            let relay_state_proof = RelayChainStateProof::new(
                T::SelfParaId::get(),
                vfp.relay_parent_storage_root,
                relay_chain_state.clone(),
            )
            .expect("Invalid relay chain state proof");

            let own_para: ParaId = T::SelfParaId::get();

            let bytes = own_para.twox_64_concat();
            // CONCAT
            let key = [PARAS_HEADS_INDEX, bytes.as_slice()].concat();

            // We might encounter enty vecs
            // We only note if we can decode
            if let Ok(head_data) = relay_state_proof.read_entry::<HeadData>(key.as_slice(), None) {
                if let Ok(mut author_header) =
                    sp_runtime::generic::Header::<BlockNumber, BlakeTwo256>::decode(
                        &mut head_data.0.as_slice(),
                    )
                {
                    let aura_digest = author_header
                        .digest_mut()
                        .logs()
                        .first()
                        .expect("Aura digest is present and is first item");

                    let (id, mut data) = aura_digest.as_pre_runtime().expect("qed");
                    if id == AURA_ENGINE_ID {
                        if let Some(slot) = InherentType::decode(&mut data).ok() {
                            if let Some(author) = T::AuthorFetcher::author_from_inherent(slot) {
                                LatestAuthor::<T>::put(author);
                            }
                        }
                    }
                }
            }

            Ok(PostDispatchInfo {
                actual_weight: Some(total_weight),
                pays_fee: Pays::No,
            })
        }

        #[pallet::call_index(0)]
        #[pallet::weight(0)]
        pub fn set_author(origin: OriginFor<T>, new: T::AccountId) -> DispatchResult {
            ensure_root(origin)?;
            LatestAuthor::<T>::put(&new);
            Self::deposit_event(Event::LatestAuthorChanged { new_author: new });
            Ok(())
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Latest author changed
        LatestAuthorChanged { new_author: T::AccountId },
    }

    #[pallet::storage]
    #[pallet::getter(fn latest_author)]
    pub(super) type LatestAuthor<T: Config> = StorageValue<_, T::AccountId, OptionQuery>;

    #[pallet::inherent]
    impl<T: Config> ProvideInherent for Pallet<T> {
        type Call = Call<T>;
        type Error = sp_inherents::MakeFatalError<()>;
        // TODO, what should we put here
        const INHERENT_IDENTIFIER: InherentIdentifier =
            tp_author_noting_inherent::INHERENT_IDENTIFIER;

        fn create_inherent(data: &InherentData) -> Option<Self::Call> {
            let data: tp_author_noting_inherent::OwnParachainInherentData = data
                .get_data(&INHERENT_IDENTIFIER)
                .ok()
                .flatten()
                .expect("there is not data to be posted; qed");

            Some(Call::set_latest_author_data { data })
        }

        fn is_inherent(call: &Self::Call) -> bool {
            matches!(call, Call::set_latest_author_data { .. })
        }
    }
}
