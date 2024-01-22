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

use {
    crate::{ParaId, Runtime, ServicesPayment, LOG_TARGET},
    frame_support::{
        pallet_prelude::ValueQuery, storage::types::StorageMap, weights::Weight, Blake2_128Concat,
    },
    pallet_configuration::{weights::WeightInfo as _, HostConfiguration},
    pallet_migrations::{GetMigrations, Migration},
    sp_core::Get,
    sp_runtime::BoundedVec,
    sp_std::{collections::btree_set::BTreeSet, marker::PhantomData, prelude::*},
};

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
        log::info!(target: LOG_TARGET, "migrate");

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
        let new_config = crate::Configuration::config();
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

pub struct MigrateServicesPaymentAddCredits<T>(pub PhantomData<T>);
impl<T> Migration for MigrateServicesPaymentAddCredits<T>
where
    T: pallet_configuration::Config,
{
    fn friendly_name(&self) -> &str {
        "TM_MigrateServicesPaymentAddCredits"
    }

    fn migrate(&self, _available_weight: Weight) -> Weight {
        // For each parachain in pallet_registrar (active, pending or pending_verification),
        // insert `MaxCreditsStored` to pallet_services_payment,
        // and mark that parachain as "given_free_credits".
        let mut para_ids = BTreeSet::new();
        let active = pallet_registrar::RegisteredParaIds::<Runtime>::get();
        let pending = pallet_registrar::PendingParaIds::<Runtime>::get();
        let pending_verification = pallet_registrar::PendingVerification::<Runtime>::get();
        // This migration ignores Paused and PendingPaused because they do not exist yet in flashbox

        para_ids.extend(active);
        para_ids.extend(pending.into_iter().flat_map(|(_session, active)| active));
        para_ids.extend(pending_verification);

        let reads = 3 + 2 * para_ids.len() as u64;
        let writes = 2 * para_ids.len() as u64;

        for para_id in para_ids {
            // 2 reads 2 writes
            ServicesPayment::give_free_credits(&para_id);
        }

        let db_weights = T::DbWeight::get();
        db_weights.reads_writes(reads, writes)
    }
}

pub struct RegistrarBootNodesStorageValuePrefix<T>(PhantomData<T>);
impl<T> frame_support::traits::StorageInstance for RegistrarBootNodesStorageValuePrefix<T> {
    const STORAGE_PREFIX: &'static str = "BootNodes";
    fn pallet_prefix() -> &'static str {
        "Registrar"
    }
}
pub type RegistrarBootNodesStorageMap<T> = StorageMap<
    RegistrarBootNodesStorageValuePrefix<T>,
    Blake2_128Concat,
    ParaId,
    //BoundedVec<BoundedVec<u8, T::MaxBootNodeUrlLen>, T::MaxBootNodes>,
    Vec<Vec<u8>>,
    ValueQuery,
>;

pub struct MigrateBootNodes<T>(pub PhantomData<T>);
impl<T> Migration for MigrateBootNodes<T>
where
    T: pallet_configuration::Config,
{
    fn friendly_name(&self) -> &str {
        "TM_MigrateBootNodes"
    }

    fn migrate(&self, _available_weight: Weight) -> Weight {
        let mut len = 0;
        for (para_id, bootnodes) in RegistrarBootNodesStorageMap::<Runtime>::drain() {
            len += 1;
            // Convert Vec<Vec<u8>> into BoundedVec<BoundedVec<u8>>
            // Cannot fail because the old storage was actually a BoundedVec with the same limit as the new one
            let bootnodes: Vec<_> = bootnodes
                .into_iter()
                .map(|bootnode| bootnode.try_into().unwrap())
                .collect();
            let bootnodes: BoundedVec<_, _> = bootnodes.try_into().unwrap();
            pallet_data_preservers::BootNodes::<Runtime>::insert(para_id, bootnodes);
        }

        let db_weights = T::DbWeight::get();
        let reads = len;
        let writes = len;
        db_weights.reads_writes(reads, writes)
    }
}

pub struct FlashboxMigrations<Runtime>(PhantomData<Runtime>);

impl<Runtime> GetMigrations for FlashboxMigrations<Runtime>
where
    Runtime: pallet_balances::Config,
    Runtime: pallet_configuration::Config,
{
    fn get_migrations() -> Vec<Box<dyn Migration>> {
        let migrate_services_payment =
            MigrateServicesPaymentAddCredits::<Runtime>(Default::default());
        let migrate_boot_nodes = MigrateBootNodes::<Runtime>(Default::default());
        let migrate_config_parathread_params =
            MigrateConfigurationParathreads::<Runtime>(Default::default());

        vec![
            Box::new(migrate_services_payment),
            Box::new(migrate_boot_nodes),
            Box::new(migrate_config_parathread_params),
        ]
    }
}
