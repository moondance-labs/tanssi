// Copyright (C) Moondance Labs Ltd.
// This file is part of Tanssi.

// Tanssi is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Tanssi is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Tanssi.  If not, see <http://www.gnu.org/licenses/>
// Copyright (C) Moondance Labs Ltd.
// This file is part of Tanssi.

// Tanssi is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Tanssi is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Tanssi.  If not, see <http://www.gnu.org/licenses/>

//! # Migrations
//!
//! This module acts as a registry where each migration is defined. Each migration should implement
//! the "Migration" trait declared in the pallet-migrations crate.

#[cfg(feature = "try-runtime")]
use frame_support::ensure;
use {
    cumulus_primitives_core::ParaId,
    frame_support::{
        migration::{clear_storage_prefix, storage_key_iter},
        pallet_prelude::GetStorageVersion,
        traits::{
            fungible::MutateHold, OnRuntimeUpgrade, PalletInfoAccess, ReservableCurrency,
            StorageVersion,
        },
        weights::Weight,
        Blake2_128Concat, BoundedVec, StoragePrefixedMap,
    },
    pallet_configuration::{weights::WeightInfo as _, HostConfiguration},
    pallet_foreign_asset_creator::{AssetId, AssetIdToForeignAsset, ForeignAssetToAssetId},
    pallet_migrations::{GetMigrations, Migration},
    pallet_registrar::HoldReason,
    sp_core::Get,
    sp_runtime::Perbill,
    sp_std::{collections::btree_set::BTreeSet, marker::PhantomData, prelude::*},
};

#[derive(
    Default,
    Clone,
    parity_scale_codec::Encode,
    parity_scale_codec::Decode,
    PartialEq,
    sp_core::RuntimeDebug,
    scale_info::TypeInfo,
)]
pub struct HostConfigurationV2 {
    pub max_collators: u32,
    pub min_orchestrator_collators: u32,
    pub max_orchestrator_collators: u32,
    pub collators_per_container: u32,
    pub full_rotation_period: u32,
    pub collators_per_parathread: u32,
    pub parathreads_per_collator: u32,
    pub target_container_chain_fullness: Perbill,
}

pub struct MigrateConfigurationAddParachainPercentage<T>(pub PhantomData<T>);
impl<T> Migration for MigrateConfigurationAddParachainPercentage<T>
where
    T: pallet_configuration::Config,
{
    fn friendly_name(&self) -> &str {
        "TM_MigrateConfigurationAddParachainPercentage"
    }

    fn migrate(&self, _available_weight: Weight) -> Weight {
        const CONFIGURATION_ACTIVE_CONFIG_KEY: &[u8] =
            &hex_literal::hex!("06de3d8a54d27e44a9d5ce189618f22db4b49d95320d9021994c850f25b8e385");
        const CONFIGURATION_PENDING_CONFIGS_KEY: &[u8] =
            &hex_literal::hex!("06de3d8a54d27e44a9d5ce189618f22d53b4123b2e186e07fb7bad5dda5f55c0");
        let default_config = HostConfiguration::default();

        // Modify active config
        let old_config: HostConfigurationV2 =
            frame_support::storage::unhashed::get(CONFIGURATION_ACTIVE_CONFIG_KEY)
                .expect("configuration.activeConfig should have value");
        let new_config = HostConfiguration {
            max_collators: old_config.max_collators,
            min_orchestrator_collators: old_config.min_orchestrator_collators,
            max_orchestrator_collators: old_config.max_orchestrator_collators,
            collators_per_container: old_config.collators_per_container,
            full_rotation_period: old_config.full_rotation_period,
            collators_per_parathread: old_config.collators_per_parathread,
            parathreads_per_collator: old_config.parathreads_per_collator,
            target_container_chain_fullness: old_config.target_container_chain_fullness,
            max_parachain_cores_percentage: default_config.max_parachain_cores_percentage,
        };
        frame_support::storage::unhashed::put(CONFIGURATION_ACTIVE_CONFIG_KEY, &new_config);

        // Modify pending configs, if any
        let old_pending_configs: Vec<(u32, HostConfigurationV2)> =
            frame_support::storage::unhashed::get(CONFIGURATION_PENDING_CONFIGS_KEY)
                .unwrap_or_default();
        let mut new_pending_configs: Vec<(u32, HostConfiguration)> = vec![];

        for (session_index, old_config) in old_pending_configs {
            let new_config = HostConfiguration {
                max_collators: old_config.max_collators,
                min_orchestrator_collators: old_config.min_orchestrator_collators,
                max_orchestrator_collators: old_config.max_orchestrator_collators,
                collators_per_container: old_config.collators_per_container,
                full_rotation_period: old_config.full_rotation_period,
                collators_per_parathread: old_config.collators_per_parathread,
                parathreads_per_collator: old_config.parathreads_per_collator,
                target_container_chain_fullness: old_config.target_container_chain_fullness,
                max_parachain_cores_percentage: default_config.max_parachain_cores_percentage,
            };
            new_pending_configs.push((session_index, new_config));
        }

        if !new_pending_configs.is_empty() {
            frame_support::storage::unhashed::put(
                CONFIGURATION_PENDING_CONFIGS_KEY,
                &new_pending_configs,
            );
        }

        <T as pallet_configuration::Config>::WeightInfo::set_config_with_u32()
    }

    /// Run a standard pre-runtime test. This works the same way as in a normal runtime upgrade.
    #[cfg(feature = "try-runtime")]
    fn pre_upgrade(&self) -> Result<Vec<u8>, sp_runtime::DispatchError> {
        const CONFIGURATION_ACTIVE_CONFIG_KEY: &[u8] =
            &hex_literal::hex!("06de3d8a54d27e44a9d5ce189618f22db4b49d95320d9021994c850f25b8e385");

        let old_config_bytes =
            frame_support::storage::unhashed::get_raw(CONFIGURATION_ACTIVE_CONFIG_KEY)
                .expect("configuration.activeConfig should have value");
        // This works because there is no enum in the v2
        assert_eq!(
            old_config_bytes.len(),
            HostConfigurationV2::default().encoded_size()
        );

        use parity_scale_codec::Encode;
        Ok((old_config_bytes).encode())
    }

    /// Run a standard post-runtime test. This works the same way as in a normal runtime upgrade.
    #[cfg(feature = "try-runtime")]
    fn post_upgrade(
        &self,
        _number_of_invulnerables: Vec<u8>,
    ) -> Result<(), sp_runtime::DispatchError> {
        let new_config = pallet_configuration::Pallet::<T>::config();
        let default_config = HostConfiguration::default();
        assert_eq!(
            new_config.max_parachain_cores_percentage,
            default_config.max_parachain_cores_percentage
        );
        Ok(())
    }
}

#[derive(
    Clone,
    parity_scale_codec::Encode,
    parity_scale_codec::Decode,
    PartialEq,
    sp_core::RuntimeDebug,
    scale_info::TypeInfo,
)]
struct HostConfigurationV1 {
    pub max_collators: u32,
    pub min_orchestrator_collators: u32,
    pub max_orchestrator_collators: u32,
    pub collators_per_container: u32,
    pub full_rotation_period: u32,
}

pub struct MigrateConfigurationParathreads<T>(pub PhantomData<T>);
impl<T> Migration for MigrateConfigurationParathreads<T>
where
    T: pallet_configuration::Config,
{
    fn friendly_name(&self) -> &str {
        "TM_MigrateConfigurationParathreads"
    }

    fn migrate(&self, _available_weight: Weight) -> Weight {
        const CONFIGURATION_ACTIVE_CONFIG_KEY: &[u8] =
            &hex_literal::hex!("06de3d8a54d27e44a9d5ce189618f22db4b49d95320d9021994c850f25b8e385");
        const CONFIGURATION_PENDING_CONFIGS_KEY: &[u8] =
            &hex_literal::hex!("06de3d8a54d27e44a9d5ce189618f22d53b4123b2e186e07fb7bad5dda5f55c0");
        let default_config = HostConfiguration::default();

        // Modify active config
        let old_config: HostConfigurationV1 =
            frame_support::storage::unhashed::get(CONFIGURATION_ACTIVE_CONFIG_KEY)
                .expect("configuration.activeConfig should have value");
        let new_config = HostConfiguration {
            max_collators: old_config.max_collators,
            min_orchestrator_collators: old_config.min_orchestrator_collators,
            max_orchestrator_collators: old_config.max_orchestrator_collators,
            collators_per_container: old_config.collators_per_container,
            full_rotation_period: old_config.full_rotation_period,
            collators_per_parathread: default_config.collators_per_parathread,
            parathreads_per_collator: default_config.parathreads_per_collator,
            target_container_chain_fullness: default_config.target_container_chain_fullness,
            max_parachain_cores_percentage: default_config.max_parachain_cores_percentage,
        };
        frame_support::storage::unhashed::put(CONFIGURATION_ACTIVE_CONFIG_KEY, &new_config);

        // Modify pending configs, if any
        let old_pending_configs: Vec<(u32, HostConfigurationV1)> =
            frame_support::storage::unhashed::get(CONFIGURATION_PENDING_CONFIGS_KEY)
                .unwrap_or_default();
        let mut new_pending_configs: Vec<(u32, HostConfiguration)> = vec![];

        for (session_index, old_config) in old_pending_configs {
            let new_config = HostConfiguration {
                max_collators: old_config.max_collators,
                min_orchestrator_collators: old_config.min_orchestrator_collators,
                max_orchestrator_collators: old_config.max_orchestrator_collators,
                collators_per_container: old_config.collators_per_container,
                full_rotation_period: old_config.full_rotation_period,
                collators_per_parathread: default_config.collators_per_parathread,
                parathreads_per_collator: default_config.parathreads_per_collator,
                target_container_chain_fullness: default_config.target_container_chain_fullness,
                max_parachain_cores_percentage: default_config.max_parachain_cores_percentage,
            };
            new_pending_configs.push((session_index, new_config));
        }

        if !new_pending_configs.is_empty() {
            frame_support::storage::unhashed::put(
                CONFIGURATION_PENDING_CONFIGS_KEY,
                &new_pending_configs,
            );
        }

        <T as pallet_configuration::Config>::WeightInfo::set_config_with_u32()
    }

    /// Run a standard pre-runtime test. This works the same way as in a normal runtime upgrade.
    #[cfg(feature = "try-runtime")]
    fn pre_upgrade(&self) -> Result<Vec<u8>, sp_runtime::DispatchError> {
        const CONFIGURATION_ACTIVE_CONFIG_KEY: &[u8] =
            &hex_literal::hex!("06de3d8a54d27e44a9d5ce189618f22db4b49d95320d9021994c850f25b8e385");

        let old_config_bytes =
            frame_support::storage::unhashed::get_raw(CONFIGURATION_ACTIVE_CONFIG_KEY)
                .expect("configuration.activeConfig should have value");
        assert_eq!(old_config_bytes.len(), 20);

        use parity_scale_codec::Encode;
        Ok((old_config_bytes).encode())
    }

    /// Run a standard post-runtime test. This works the same way as in a normal runtime upgrade.
    #[cfg(feature = "try-runtime")]
    fn post_upgrade(
        &self,
        _number_of_invulnerables: Vec<u8>,
    ) -> Result<(), sp_runtime::DispatchError> {
        let new_config = pallet_configuration::Pallet::<T>::config();
        let default_config = HostConfiguration::default();
        assert_eq!(
            new_config.collators_per_parathread,
            default_config.collators_per_parathread
        );
        assert_eq!(
            new_config.parathreads_per_collator,
            default_config.collators_per_parathread
        );
        assert_eq!(
            new_config.target_container_chain_fullness,
            default_config.target_container_chain_fullness
        );

        Ok(())
    }
}

pub struct MigrateServicesPaymentAddCollatorAssignmentCredits<T>(pub PhantomData<T>);
impl<T> Migration for MigrateServicesPaymentAddCollatorAssignmentCredits<T>
where
    T: pallet_services_payment::Config + pallet_registrar::Config,
{
    fn friendly_name(&self) -> &str {
        "TM_MigrateServicesPaymentAddCollatorAssignmentCredits"
    }

    fn migrate(&self, _available_weight: Weight) -> Weight {
        // For each parachain in pallet_registrar (active, pending or pending_verification),
        // insert `MaxCreditsStored` to pallet_services_payment,
        // and mark that parachain as "given_free_credits".
        let mut para_ids = BTreeSet::new();
        let active = pallet_registrar::RegisteredParaIds::<T>::get();
        let pending = pallet_registrar::PendingParaIds::<T>::get();

        let paused = pallet_registrar::Paused::<T>::get();
        para_ids.extend(active);
        para_ids.extend(pending.into_iter().flat_map(|(_session, active)| active));
        para_ids.extend(paused);

        let reads = 3 + 2 * para_ids.len() as u64;
        let writes = 2 * para_ids.len() as u64;

        for para_id in para_ids {
            // 2 reads 2 writes
            pallet_services_payment::Pallet::<T>::set_free_collator_assignment_credits(
                &para_id,
                T::FreeCollatorAssignmentCredits::get(),
            );
        }

        let db_weights = T::DbWeight::get();
        db_weights.reads_writes(reads, writes)
    }
    /// Run a standard pre-runtime test. This works the same way as in a normal runtime upgrade.
    #[cfg(feature = "try-runtime")]
    fn pre_upgrade(&self) -> Result<Vec<u8>, sp_runtime::DispatchError> {
        let mut para_ids = BTreeSet::new();
        let active = pallet_registrar::RegisteredParaIds::<T>::get();
        let pending = pallet_registrar::PendingParaIds::<T>::get();
        let paused = pallet_registrar::Paused::<T>::get();
        para_ids.extend(active);
        para_ids.extend(pending.into_iter().flat_map(|(_session, active)| active));
        para_ids.extend(paused);

        for para_id in para_ids {
            assert!(
                pallet_services_payment::CollatorAssignmentCredits::<T>::get(para_id).is_none()
            );
        }

        Ok(vec![])
    }

    // Run a standard post-runtime test. This works the same way as in a normal runtime upgrade.
    #[cfg(feature = "try-runtime")]
    fn post_upgrade(&self, _result: Vec<u8>) -> Result<(), sp_runtime::DispatchError> {
        let mut para_ids = BTreeSet::new();
        let active = pallet_registrar::RegisteredParaIds::<T>::get();
        let pending = pallet_registrar::PendingParaIds::<T>::get();
        let paused = pallet_registrar::Paused::<T>::get();
        para_ids.extend(active);
        para_ids.extend(pending.into_iter().flat_map(|(_session, active)| active));
        para_ids.extend(paused);

        for para_id in para_ids {
            assert_eq!(
                pallet_services_payment::CollatorAssignmentCredits::<T>::get(para_id),
                Some(T::FreeCollatorAssignmentCredits::get())
            );
        }

        Ok(())
    }
}

pub struct PolkadotXcmMigrationFixVersion<T, PolkadotXcm>(pub PhantomData<(T, PolkadotXcm)>);
impl<T, PolkadotXcm> Migration for PolkadotXcmMigrationFixVersion<T, PolkadotXcm>
where
    PolkadotXcm: GetStorageVersion + PalletInfoAccess,
    T: cumulus_pallet_xcmp_queue::Config,
{
    fn friendly_name(&self) -> &str {
        "MM_PolkadotXcmMigrationFixVersion"
    }

    fn migrate(&self, _available_weight: Weight) -> Weight {
        if <PolkadotXcm as GetStorageVersion>::on_chain_storage_version() == 0 {
            StorageVersion::new(1).put::<PolkadotXcm>();
            return T::DbWeight::get().writes(1);
        }
        Weight::default()
    }

    #[cfg(feature = "try-runtime")]
    fn pre_upgrade(&self) -> Result<Vec<u8>, sp_runtime::DispatchError> {
        ensure!(
            <PolkadotXcm as GetStorageVersion>::on_chain_storage_version() == 0,
            "PolkadotXcm storage version should be 0"
        );
        Ok(vec![])
    }

    // Run a standard post-runtime test. This works the same way as in a normal runtime upgrade.
    #[cfg(feature = "try-runtime")]
    fn post_upgrade(&self, _state: Vec<u8>) -> Result<(), sp_runtime::DispatchError> {
        ensure!(
            <PolkadotXcm as GetStorageVersion>::on_chain_storage_version() == 1,
            "PolkadotXcm storage version should be 1"
        );
        Ok(())
    }
}

pub struct XcmpQueueMigrationFixVersion<T, XcmpQueue>(pub PhantomData<(T, XcmpQueue)>);
impl<T, XcmpQueue> Migration for XcmpQueueMigrationFixVersion<T, XcmpQueue>
where
    XcmpQueue: GetStorageVersion + PalletInfoAccess,
    T: cumulus_pallet_xcmp_queue::Config,
{
    fn friendly_name(&self) -> &str {
        "MM_XcmpQueueMigrationFixVersion"
    }

    fn migrate(&self, _available_weight: Weight) -> Weight {
        if <XcmpQueue as GetStorageVersion>::on_chain_storage_version() == 0 {
            StorageVersion::new(2).put::<XcmpQueue>();
            return T::DbWeight::get().writes(1);
        }
        Weight::default()
    }

    #[cfg(feature = "try-runtime")]
    fn pre_upgrade(&self) -> Result<Vec<u8>, sp_runtime::DispatchError> {
        ensure!(
            <XcmpQueue as GetStorageVersion>::on_chain_storage_version() == 0,
            "XcmpQueue storage version should be 0"
        );
        Ok(vec![])
    }

    // Run a standard post-runtime test. This works the same way as in a normal runtime upgrade.
    #[cfg(feature = "try-runtime")]
    fn post_upgrade(&self, _state: Vec<u8>) -> Result<(), sp_runtime::DispatchError> {
        // Greater than because the post_upgrade is run after all the migrations, so
        // it can be greater if the following XcmpQueue migrations are applied in the
        // same runtime
        ensure!(
            <XcmpQueue as GetStorageVersion>::on_chain_storage_version() >= 2,
            "XcmpQueue storage version should be at least 2"
        );
        Ok(())
    }
}

pub struct XcmpQueueMigrationV3<T>(pub PhantomData<T>);
impl<T> Migration for XcmpQueueMigrationV3<T>
where
    T: cumulus_pallet_xcmp_queue::Config,
{
    fn friendly_name(&self) -> &str {
        "MM_XcmpQueueMigrationV3"
    }

    fn migrate(&self, _available_weight: Weight) -> Weight {
        cumulus_pallet_xcmp_queue::migration::v3::MigrationToV3::<T>::on_runtime_upgrade()
    }

    // #[cfg(feature = "try-runtime")]
    // let mut pre_upgrade_result: Vec<u8>;

    #[cfg(feature = "try-runtime")]
    fn pre_upgrade(&self) -> Result<Vec<u8>, sp_runtime::DispatchError> {
        cumulus_pallet_xcmp_queue::migration::v3::MigrationToV3::<T>::pre_upgrade()
    }

    // Run a standard post-runtime test. This works the same way as in a normal runtime upgrade.
    #[cfg(feature = "try-runtime")]
    fn post_upgrade(&self, state: Vec<u8>) -> Result<(), sp_runtime::DispatchError> {
        cumulus_pallet_xcmp_queue::migration::v3::MigrationToV3::<T>::post_upgrade(state)
    }
}

pub struct XcmpQueueMigrationV4<T>(pub PhantomData<T>);
impl<T> Migration for XcmpQueueMigrationV4<T>
where
    T: cumulus_pallet_xcmp_queue::Config,
{
    fn friendly_name(&self) -> &str {
        "MM_XcmpQueueMigrationV4"
    }

    fn migrate(&self, _available_weight: Weight) -> Weight {
        cumulus_pallet_xcmp_queue::migration::v4::MigrationToV4::<T>::on_runtime_upgrade()
    }

    #[cfg(feature = "try-runtime")]
    fn pre_upgrade(&self) -> Result<Vec<u8>, sp_runtime::DispatchError> {
        cumulus_pallet_xcmp_queue::migration::v4::MigrationToV4::<T>::pre_upgrade()
    }

    // Run a standard post-runtime test. This works the same way as in a normal runtime upgrade.
    #[cfg(feature = "try-runtime")]
    fn post_upgrade(&self, state: Vec<u8>) -> Result<(), sp_runtime::DispatchError> {
        cumulus_pallet_xcmp_queue::migration::v4::MigrationToV4::<T>::post_upgrade(state)
    }
}

pub struct RegistrarPendingVerificationValueToMap<T>(pub PhantomData<T>);
impl<T> Migration for RegistrarPendingVerificationValueToMap<T>
where
    T: pallet_registrar::Config,
{
    fn friendly_name(&self) -> &str {
        "TM_RegistrarPendingVerificationValueToMap"
    }

    fn migrate(&self, _available_weight: Weight) -> Weight {
        let para_ids: Vec<ParaId> = frame_support::storage::unhashed::take(
            &frame_support::storage::storage_prefix(b"Registrar", b"PendingVerification"),
        )
        .unwrap_or_default();

        let total_weight = T::DbWeight::get()
            .reads(1)
            .saturating_add(T::DbWeight::get().writes(1));

        for para_id in para_ids {
            total_weight.saturating_add(T::DbWeight::get().writes(1));
            pallet_registrar::PendingVerification::<T>::insert(para_id, ());
        }

        total_weight
    }

    #[cfg(feature = "try-runtime")]
    fn pre_upgrade(&self) -> Result<Vec<u8>, sp_runtime::DispatchError> {
        Ok(vec![])
    }

    // Run a standard post-runtime test. This works the same way as in a normal runtime upgrade.
    #[cfg(feature = "try-runtime")]
    fn post_upgrade(&self, _state: Vec<u8>) -> Result<(), sp_runtime::DispatchError> {
        Ok(())
    }
}

pub struct RegistrarParaManagerMigration<T>(pub PhantomData<T>);
impl<T> Migration for RegistrarParaManagerMigration<T>
where
    T: pallet_registrar::Config,
{
    fn friendly_name(&self) -> &str {
        "TM_RegistrarParaManagerMigration"
    }

    fn migrate(&self, _available_weight: Weight) -> Weight {
        let mut total_weight = Weight::default();
        for (para_id, deposit) in pallet_registrar::RegistrarDeposit::<T>::iter() {
            pallet_registrar::ParaManager::<T>::insert(para_id, deposit.creator);
            total_weight = total_weight.saturating_add(
                T::DbWeight::get()
                    .reads(1)
                    .saturating_add(T::DbWeight::get().writes(1)),
            );
        }

        total_weight
    }

    #[cfg(feature = "try-runtime")]
    fn pre_upgrade(&self) -> Result<Vec<u8>, sp_runtime::DispatchError> {
        Ok(vec![])
    }

    #[cfg(feature = "try-runtime")]
    fn post_upgrade(&self, _state: Vec<u8>) -> Result<(), sp_runtime::DispatchError> {
        Ok(())
    }
}

pub struct RegistrarReserveToHoldMigration<T>(pub PhantomData<T>);
impl<T> Migration for RegistrarReserveToHoldMigration<T>
where
    T: pallet_registrar::Config,
    T: pallet_balances::Config,
    <T as pallet_balances::Config>::RuntimeHoldReason: From<pallet_registrar::HoldReason>,
    <T as pallet_balances::Config>::Balance: From<
        <<T as pallet_registrar::Config>::Currency as frame_support::traits::fungible::Inspect<
            T::AccountId,
        >>::Balance,
    >,
    <T as pallet_balances::Config>::Balance: From<u128>,
{
    fn friendly_name(&self) -> &str {
        "TM_RegistrarReserveToHoldMigration"
    }

    fn migrate(&self, _available_weight: Weight) -> Weight {
        let mut total_weight = Weight::default();
        for (_para_id, deposit) in pallet_registrar::RegistrarDeposit::<T>::iter() {
            pallet_balances::Pallet::<T>::unreserve(&deposit.creator, deposit.deposit.into());
            let _ = pallet_balances::Pallet::<T>::hold(
                &HoldReason::RegistrarDeposit.into(),
                &deposit.creator,
                deposit.deposit.into(),
            );
            total_weight = total_weight.saturating_add(T::DbWeight::get().reads_writes(2, 2));
        }

        total_weight
    }

    #[cfg(feature = "try-runtime")]
    fn pre_upgrade(&self) -> Result<Vec<u8>, sp_runtime::DispatchError> {
        use frame_support::traits::fungible::InspectHold;

        for (_para_id, deposit) in pallet_registrar::RegistrarDeposit::<T>::iter() {
            ensure!(
                pallet_balances::Pallet::<T>::reserved_balance(&deposit.creator)
                    >= deposit.deposit.into(),
                "Reserved balanced cannot be less than deposit amount"
            );

            ensure!(
                pallet_balances::Pallet::<T>::balance_on_hold(
                    &HoldReason::RegistrarDeposit.into(),
                    &deposit.creator
                ) == 0u128.into(),
                "Balance on hold for RegistrarDeposit should be 0"
            );
        }
        Ok(vec![])
    }

    #[cfg(feature = "try-runtime")]
    fn post_upgrade(&self, _state: Vec<u8>) -> Result<(), sp_runtime::DispatchError> {
        use frame_support::traits::fungible::InspectHold;

        for (_para_id, deposit) in pallet_registrar::RegistrarDeposit::<T>::iter() {
            ensure!(
                pallet_balances::Pallet::<T>::balance_on_hold(
                    &HoldReason::RegistrarDeposit.into(),
                    &deposit.creator
                ) >= deposit.deposit.into(),
                "Balance on hold for RegistrarDeposit should be deposit"
            );
        }

        Ok(())
    }
}
pub struct MigrateToLatestXcmVersion<Runtime>(PhantomData<Runtime>);
impl<Runtime> Migration for MigrateToLatestXcmVersion<Runtime>
where
    pallet_xcm::migration::MigrateToLatestXcmVersion<Runtime>:
        frame_support::traits::OnRuntimeUpgrade,
{
    fn friendly_name(&self) -> &str {
        "MM_MigrateToLatestXcmVersion"
    }

    fn migrate(&self, _available_weight: Weight) -> Weight {
        pallet_xcm::migration::MigrateToLatestXcmVersion::<Runtime>::on_runtime_upgrade()
    }

    #[cfg(feature = "try-runtime")]
    fn pre_upgrade(&self) -> Result<Vec<u8>, sp_runtime::DispatchError> {
        pallet_xcm::migration::MigrateToLatestXcmVersion::<Runtime>::pre_upgrade()
    }

    #[cfg(feature = "try-runtime")]
    fn post_upgrade(&self, state: Vec<u8>) -> Result<(), sp_runtime::DispatchError> {
        pallet_xcm::migration::MigrateToLatestXcmVersion::<Runtime>::post_upgrade(state)
    }
}

pub struct DataPreserversAssignmentsMigration<T>(pub PhantomData<T>);
impl<T> Migration for DataPreserversAssignmentsMigration<T>
where
    T: pallet_data_preservers::Config + pallet_registrar::Config,
    T::AccountId: From<[u8; 32]>,
{
    fn friendly_name(&self) -> &str {
        "TM_DataPreserversAssignmentsMigration"
    }

    fn migrate(&self, _available_weight: Weight) -> Weight {
        use {
            frame_support::BoundedBTreeSet,
            frame_system::RawOrigin,
            pallet_data_preservers::{AssignmentPayment, ParaIdsFilter, Profile, ProfileMode},
        };

        let mut total_weight = Weight::default();

        let (request, _extra, witness) = T::AssignmentPayment::free_variant_values()
            .expect("free variant values are necessary to perform migration");

        let dummy_profile_owner = T::AccountId::from([0u8; 32]);

        let pallet_prefix: &[u8] = b"DataPreservers";
        let storage_item_prefix: &[u8] = b"BootNodes";
        let bootnodes_storage: Vec<_> = storage_key_iter::<
            ParaId,
            BoundedVec<BoundedVec<u8, T::MaxNodeUrlLen>, T::MaxAssignmentsPerParaId>,
            Blake2_128Concat,
        >(pallet_prefix, storage_item_prefix)
        .collect();

        total_weight = total_weight.saturating_add(
            T::DbWeight::get()
                .reads(bootnodes_storage.len() as u64)
                .saturating_add(T::DbWeight::get().writes(bootnodes_storage.len() as u64)),
        );

        for (para_id, bootnodes) in bootnodes_storage {
            for bootnode_url in bootnodes {
                let profile = Profile {
                    url: bootnode_url,
                    para_ids: ParaIdsFilter::Whitelist({
                        let mut set = BoundedBTreeSet::new();
                        set.try_insert(para_id).expect("to be in bound");
                        set
                    }),
                    mode: ProfileMode::Bootnode,
                    assignment_request: request.clone(),
                };

                let profile_id = pallet_data_preservers::NextProfileId::<T>::get();

                if let Some(weight) = pallet_data_preservers::Pallet::<T>::force_create_profile(
                    RawOrigin::Root.into(),
                    profile,
                    dummy_profile_owner.clone(),
                )
                .expect("to create profile")
                .actual_weight
                {
                    total_weight = total_weight.saturating_add(weight);
                }

                if let Some(weight) = pallet_data_preservers::Pallet::<T>::force_start_assignment(
                    RawOrigin::Root.into(),
                    profile_id,
                    para_id,
                    witness.clone(),
                )
                .expect("to start assignment")
                .actual_weight
                {
                    total_weight = total_weight.saturating_add(weight);
                }
            }
        }

        let _ = clear_storage_prefix(pallet_prefix, storage_item_prefix, &[], None, None);

        total_weight
    }

    #[cfg(feature = "try-runtime")]
    fn pre_upgrade(&self) -> Result<Vec<u8>, sp_runtime::DispatchError> {
        use parity_scale_codec::Encode;

        let pallet_prefix: &[u8] = b"DataPreservers";
        let storage_item_prefix: &[u8] = b"BootNodes";
        let state: Vec<_> = storage_key_iter::<
            ParaId,
            BoundedVec<BoundedVec<u8, T::MaxNodeUrlLen>, T::MaxAssignmentsPerParaId>,
            Blake2_128Concat,
        >(pallet_prefix, storage_item_prefix)
        .collect();

        Ok(state.encode())
    }

    #[cfg(feature = "try-runtime")]
    fn post_upgrade(&self, state: Vec<u8>) -> Result<(), sp_runtime::DispatchError> {
        use parity_scale_codec::Decode;

        let pallet_prefix: &[u8] = b"DataPreservers";
        let storage_item_prefix: &[u8] = b"BootNodes";

        let pre_state: Vec<(ParaId, Vec<Vec<u8>>)> =
            Decode::decode(&mut &state[..]).expect("state to be decoded properly");

        for (para_id, bootnodes) in pre_state {
            let assignments = pallet_data_preservers::Assignments::<T>::get(para_id);
            assert_eq!(assignments.len(), bootnodes.len());

            let profiles: Vec<_> =
                pallet_data_preservers::Pallet::<T>::assignments_profiles(para_id).collect();

            for bootnode in bootnodes {
                assert_eq!(
                    profiles
                        .iter()
                        .filter(|profile| profile.url == bootnode)
                        .count(),
                    1
                );
            }
        }

        assert_eq!(
            storage_key_iter::<
                ParaId,
                BoundedVec<BoundedVec<u8, T::MaxNodeUrlLen>, T::MaxAssignmentsPerParaId>,
                Blake2_128Concat,
            >(pallet_prefix, storage_item_prefix)
            .count(),
            0
        );

        Ok(())
    }
}

pub struct ForeignAssetCreatorMigration<Runtime>(pub PhantomData<Runtime>);

impl<Runtime> Migration for ForeignAssetCreatorMigration<Runtime>
where
    Runtime: pallet_foreign_asset_creator::Config,
    <Runtime as pallet_foreign_asset_creator::Config>::ForeignAsset:
        TryFrom<staging_xcm::v3::MultiLocation>,
{
    fn friendly_name(&self) -> &str {
        "TM_ForeignAssetCreatorMigration"
    }

    fn migrate(&self, _available_weight: Weight) -> Weight {
        use frame_support::pallet_prelude::*;

        use staging_xcm::v3::MultiLocation as OldLocation;

        let pallet_prefix = AssetIdToForeignAsset::<Runtime>::pallet_prefix();
        let asset_id_to_foreign_asset_storage_prefix =
            AssetIdToForeignAsset::<Runtime>::storage_prefix();
        let foreign_asset_to_asset_id_prefix = ForeignAssetToAssetId::<Runtime>::storage_prefix();

        // Data required to migrate ForeignAsset values
        // Read all the data into memory.
        let asset_id_to_foreign_asset_data: Vec<_> =
            storage_key_iter::<AssetId<Runtime>, OldLocation, Blake2_128Concat>(
                pallet_prefix,
                asset_id_to_foreign_asset_storage_prefix,
            )
            .drain()
            .collect();

        // Data required to migrate ForeignAsset keys
        let foreign_asset_to_asset_id_data: Vec<_> =
            storage_key_iter::<OldLocation, AssetId<Runtime>, Blake2_128Concat>(
                pallet_prefix,
                foreign_asset_to_asset_id_prefix,
            )
            .drain()
            .collect();

        let migrated_count = asset_id_to_foreign_asset_data
            .len()
            .saturating_add(foreign_asset_to_asset_id_data.len());

        log::info!("Migrating {:?} elements", migrated_count);

        // Write to the new storage with removed and added fields
        for (asset_id, old_location) in asset_id_to_foreign_asset_data {
            if let Ok(new_location) = Runtime::ForeignAsset::try_from(old_location) {
                AssetIdToForeignAsset::<Runtime>::insert(asset_id, new_location);
            } else {
                log::warn!("Location could not be converted safely to xcmV4")
            }
        }

        for (old_location, asset_id) in foreign_asset_to_asset_id_data {
            if let Ok(new_location) = Runtime::ForeignAsset::try_from(old_location) {
                ForeignAssetToAssetId::<Runtime>::insert(new_location, asset_id);
            } else {
                log::warn!("Location could not be converted safely to xcmV4")
            }
        }

        // One db read and one db write per element, plus the on-chain storage
        Runtime::DbWeight::get().reads_writes(migrated_count as u64, 2 * migrated_count as u64)
    }

    #[cfg(feature = "try-runtime")]
    fn pre_upgrade(&self) -> Result<Vec<u8>, sp_runtime::DispatchError> {
        Ok(vec![])
    }

    #[cfg(feature = "try-runtime")]
    fn post_upgrade(&self, _state: Vec<u8>) -> Result<(), sp_runtime::DispatchError> {
        Ok(())
    }
}

pub struct FlashboxMigrations<Runtime>(PhantomData<Runtime>);

impl<Runtime> GetMigrations for FlashboxMigrations<Runtime>
where
    Runtime: pallet_balances::Config,
    Runtime: pallet_configuration::Config,
    Runtime: pallet_registrar::Config,
    Runtime: pallet_data_preservers::Config,
    Runtime: pallet_services_payment::Config,
    Runtime: pallet_data_preservers::Config,
    Runtime::AccountId: From<[u8; 32]>,
    <Runtime as pallet_balances::Config>::RuntimeHoldReason: From<pallet_registrar::HoldReason>,
    <Runtime as pallet_balances::Config>::Balance: From<<<Runtime as pallet_registrar::Config>::Currency as frame_support::traits::fungible::Inspect<Runtime::AccountId>>::Balance>,
    <Runtime as pallet_balances::Config>::Balance: From<u128>,
{
    fn get_migrations() -> Vec<Box<dyn Migration>> {
        //let migrate_services_payment =
        //    MigrateServicesPaymentAddCredits::<Runtime>(Default::default());
        //let migrate_boot_nodes = MigrateBootNodes::<Runtime>(Default::default());
        let migrate_config_parathread_params =
            MigrateConfigurationParathreads::<Runtime>(Default::default());

        let migrate_add_collator_assignment_credits =
            MigrateServicesPaymentAddCollatorAssignmentCredits::<Runtime>(Default::default());
        let migrate_registrar_pending_verification =
            RegistrarPendingVerificationValueToMap::<Runtime>(Default::default());
        let migrate_registrar_manager =
            RegistrarParaManagerMigration::<Runtime>(Default::default());
        let migrate_data_preservers_assignments =
            DataPreserversAssignmentsMigration::<Runtime>(Default::default());
        let migrate_registrar_reserves = RegistrarReserveToHoldMigration::<Runtime>(Default::default());
        let migrate_config_max_parachain_percentage = MigrateConfigurationAddParachainPercentage::<Runtime>(Default::default());

        vec![
            // Applied in runtime 400
            //Box::new(migrate_services_payment),
            // Applied in runtime 400
            //Box::new(migrate_boot_nodes),
            // Applied in runtime 400
            Box::new(migrate_config_parathread_params),
            Box::new(migrate_add_collator_assignment_credits),
            Box::new(migrate_registrar_pending_verification),
            Box::new(migrate_registrar_manager),
            Box::new(migrate_data_preservers_assignments),
            Box::new(migrate_registrar_reserves),
            Box::new(migrate_config_max_parachain_percentage),
        ]
    }
}

pub struct DanceboxMigrations<Runtime>(PhantomData<Runtime>);

impl<Runtime> GetMigrations for DanceboxMigrations<Runtime>
where
    Runtime: pallet_pooled_staking::Config,
    Runtime: pallet_registrar::Config,
    Runtime: pallet_balances::Config,
    Runtime: pallet_configuration::Config,
    Runtime: pallet_services_payment::Config,
    Runtime: cumulus_pallet_xcmp_queue::Config,
    Runtime: pallet_data_preservers::Config,
    Runtime: pallet_xcm::Config,
    <Runtime as pallet_balances::Config>::RuntimeHoldReason:
        From<pallet_pooled_staking::HoldReason>,
    Runtime: pallet_foreign_asset_creator::Config,
    <Runtime as pallet_foreign_asset_creator::Config>::ForeignAsset:
        TryFrom<staging_xcm::v3::MultiLocation>,
    <Runtime as pallet_balances::Config>::RuntimeHoldReason:
        From<pallet_registrar::HoldReason>,
    Runtime::AccountId: From<[u8; 32]>,
    <Runtime as pallet_balances::Config>::Balance: From<<<Runtime as pallet_registrar::Config>::Currency as frame_support::traits::fungible::Inspect<Runtime::AccountId>>::Balance>,
    <Runtime as pallet_balances::Config>::Balance: From<u128>,
{
    fn get_migrations() -> Vec<Box<dyn Migration>> {
        // let migrate_invulnerables = MigrateInvulnerables::<Runtime>(Default::default());
        // let migrate_holds = MigrateHoldReason::<Runtime>(Default::default());
        // let migrate_config = MigrateConfigurationFullRotationPeriod::<Runtime>(Default::default());
        // let migrate_xcm = PolkadotXcmMigration::<Runtime>(Default::default());
        // let migrate_xcmp_queue = XcmpQueueMigration::<Runtime>(Default::default());
        // let migrate_services_payment =
        //     MigrateServicesPaymentAddCredits::<Runtime>(Default::default());
        // let migrate_boot_nodes = MigrateBootNodes::<Runtime>(Default::default());
        // let migrate_hold_reason_runtime_enum =
        //     MigrateHoldReasonRuntimeEnum::<Runtime>(Default::default());

        let migrate_config_parathread_params =
            MigrateConfigurationParathreads::<Runtime>(Default::default());
        let migrate_add_collator_assignment_credits =
            MigrateServicesPaymentAddCollatorAssignmentCredits::<Runtime>(Default::default());
        let migrate_xcmp_queue_v4 = XcmpQueueMigrationV4::<Runtime>(Default::default());
        let migrate_registrar_pending_verification =
            RegistrarPendingVerificationValueToMap::<Runtime>(Default::default());
        let migrate_registrar_manager =
            RegistrarParaManagerMigration::<Runtime>(Default::default());
        let migrate_data_preservers_assignments =
            DataPreserversAssignmentsMigration::<Runtime>(Default::default());

        let migrate_pallet_xcm_v4 = MigrateToLatestXcmVersion::<Runtime>(Default::default());
        let foreign_asset_creator_migration =
            ForeignAssetCreatorMigration::<Runtime>(Default::default());
        let migrate_registrar_reserves = RegistrarReserveToHoldMigration::<Runtime>(Default::default());
        let migrate_config_max_parachain_percentage = MigrateConfigurationAddParachainPercentage::<Runtime>(Default::default());

        vec![
            // Applied in runtime 200
            //Box::new(migrate_invulnerables),
            // Applied in runtime 200
            //Box::new(migrate_holds),
            // Applied in runtime 300
            //Box::new(migrate_config),
            // Applied in runtime 300
            //Box::new(migrate_xcm),
            // Applied in runtime 300
            //Box::new(migrate_xcmp_queue),
            // Applied in runtime 400
            //Box::new(migrate_services_payment),
            // Applied in runtime 400
            //Box::new(migrate_hold_reason_runtime_enum),
            // Applied in runtime 400
            //Box::new(migrate_boot_nodes),
            Box::new(migrate_config_parathread_params),
            Box::new(migrate_add_collator_assignment_credits),
            Box::new(migrate_xcmp_queue_v4),
            Box::new(migrate_registrar_pending_verification),
            Box::new(migrate_registrar_manager),
            Box::new(migrate_pallet_xcm_v4),
            Box::new(foreign_asset_creator_migration),
            Box::new(migrate_data_preservers_assignments),
            Box::new(migrate_registrar_reserves),
            Box::new(migrate_config_max_parachain_percentage)
        ]
    }
}

pub struct DancelightMigrations<Runtime>(PhantomData<Runtime>);

impl<Runtime> GetMigrations for DancelightMigrations<Runtime> {
    fn get_migrations() -> Vec<Box<dyn Migration>> {
        vec![]
    }
}
