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

use cumulus_primitives_core::Weight;
use pallet_configuration::{HostConfiguration, WeightInfo as _};
use pallet_migrations::Migration;
use parity_scale_codec::Decode;
use sp_std::marker::PhantomData;
use sp_std::vec::Vec;

// Separate
use sp_std::vec;

pub trait MigrateStructToLatest<T> {
    fn expand_with_default_values(self, default_config: &T) -> T;
}

pub struct GenericMigrateConfiguration<T, HostConfigurationV> {
    pub name: &'static str,
    pub default: HostConfiguration,
    pub phantom: PhantomData<(T, HostConfigurationV)>,
}

impl<T, HostConfigurationV> Migration for GenericMigrateConfiguration<T, HostConfigurationV>
where
    T: pallet_configuration::Config,
    HostConfigurationV: Decode + MigrateStructToLatest<HostConfiguration>,
{
    fn friendly_name(&self) -> &str {
        self.name
    }

    fn migrate(&self, _available_weight: Weight) -> Weight {
        // Modify active config
        let old_config: HostConfigurationV = frame_support::storage::unhashed::get(
            &pallet_configuration::ActiveConfig::<T>::hashed_key(),
        )
        .expect("configuration.activeConfig should have value");
        let new_config = HostConfigurationV::expand_with_default_values(old_config, &self.default);
        frame_support::storage::unhashed::put(
            &pallet_configuration::ActiveConfig::<T>::hashed_key(),
            &new_config,
        );

        // Modify pending configs, if any
        let old_pending_configs: Vec<(u32, HostConfigurationV)> =
            frame_support::storage::unhashed::get(
                &pallet_configuration::PendingConfigs::<T>::hashed_key(),
            )
            .unwrap_or_default();
        let mut new_pending_configs: Vec<(u32, HostConfiguration)> = vec![];

        for (session_index, old_config) in old_pending_configs {
            let new_config =
                HostConfigurationV::expand_with_default_values(old_config, &self.default);
            new_pending_configs.push((session_index, new_config));
        }

        if !new_pending_configs.is_empty() {
            frame_support::storage::unhashed::put(
                &pallet_configuration::PendingConfigs::<T>::hashed_key(),
                &new_pending_configs,
            );
        }

        <T as pallet_configuration::Config>::WeightInfo::set_config_with_u32()
    }

    /// Run a standard pre-runtime test. This works the same way as in a normal runtime upgrade.
    #[cfg(feature = "try-runtime")]
    fn pre_upgrade(&self) -> Result<Vec<u8>, sp_runtime::DispatchError> {
        Ok(vec![])
    }

    /// Run a standard post-runtime test. This works the same way as in a normal runtime upgrade.
    #[cfg(feature = "try-runtime")]
    fn post_upgrade(
        &self,
        _number_of_invulnerables: Vec<u8>,
    ) -> Result<(), sp_runtime::DispatchError> {
        // TODO: add a generic callback here?
        /*
        let new_config = pallet_configuration::Pallet::<T>::config();
        let default_config = HostConfiguration::default();

        assert_eq!(
            new_config.max_parachain_cores_percentage,
            default_config.max_parachain_cores_percentage
        );
         */
        Ok(())
    }
}
