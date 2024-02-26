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

#![cfg_attr(not(feature = "std"), no_std)]

#[doc(hidden)]
pub mod deps {
    pub use {
        frame_support, impls::impls, pallet_author_inherent, pallet_cc_authorities_noting,
        pallet_timestamp,
    };
}

pub trait Config {
    const SLOT_DURATION: u64;
    type TimestampWeights: pallet_timestamp::weights::WeightInfo;
    type AuthorInherentWeights: pallet_author_inherent::weights::WeightInfo;
    type AuthoritiesNotingWeights: pallet_cc_authorities_noting::weights::WeightInfo;
}

/// Implements Config traits for `pallet_author_inherent`, `pallet_timestamp` and
/// `pallet_cc_authorities_noting` with the proper parameters to be compatible with Tanssi.
/// Requires to implement the [`Config`] trait on the runtime.
/// It generates a test that will fail if the pallets above are not listed in `construct_runtime!`.
#[macro_export]
macro_rules! impl_tanssi_pallets_config {
    (
        $runtime:ident
    ) => {
        // `const _:() = { ... }` allows to import and define types that will not leak into the macro
        // call site.
        const _: () = {
            use $crate::deps::*;

            impl pallet_author_inherent::Config for $runtime {
                type AuthorId = NimbusId;
                type AccountLookup = tp_consensus::NimbusLookUp;
                type CanAuthor = pallet_cc_authorities_noting::CanAuthor<$runtime>;
                type SlotBeacon = tp_consensus::AuraDigestSlotBeacon<$runtime>;
                type WeightInfo = <$runtime as $crate::Config>::AuthorInherentWeights;
            }

            impl pallet_timestamp::Config for $runtime {
                /// A timestamp: milliseconds since the unix epoch.
                type Moment = u64;
                type OnTimestampSet = tp_consensus::OnTimestampSet<
                    <Self as pallet_author_inherent::Config>::SlotBeacon,
                    ConstU64<{ <$runtime as $crate::Config>::SLOT_DURATION }>,
                >;
                type MinimumPeriod = ConstU64<{ <$runtime as $crate::Config>::SLOT_DURATION / 2 }>;
                type WeightInfo = <$runtime as $crate::Config>::TimestampWeights;
            }

            impl pallet_cc_authorities_noting::Config for $runtime {
                type RuntimeEvent = RuntimeEvent;
                type SelfParaId = parachain_info::Pallet<$runtime>;
                type RelayChainStateProvider =
                    cumulus_pallet_parachain_system::RelaychainDataProvider<Self>;
                type AuthorityId = NimbusId;
                type WeightInfo = <$runtime as $crate::Config>::AuthoritiesNotingWeights;

                #[cfg(feature = "runtime-benchmarks")]
                type BenchmarkHelper =
                    pallet_cc_authorities_noting::benchmarks::NimbusIdBenchmarkHelper;
            }
        };

        #[test]
        fn __impl_tanssi_pallets_config_tests() {
            use $crate::deps::{frame_support::traits::PalletInfo, impls};

            let runtime_name = stringify!($runtime);

            fn is_pallet_installed<P: 'static>() -> bool {
                <$runtime as frame_system::Config>::PalletInfo::index::<P>().is_some()
            }

            assert!(
                impls!($runtime: $crate::Config),
                "{runtime_name} must impl tp_impl_tanssi_pallets_config::Config"
            );
            assert!(
                is_pallet_installed::<pallet_author_inherent::Pallet::<$runtime>>(),
                "pallet_author_inherent is not installed in {runtime_name}"
            );
            assert!(
                is_pallet_installed::<pallet_cc_authorities_noting::Pallet::<$runtime>>(),
                "pallet_cc_authorities_noting is not installed in {runtime_name}"
            );
            assert!(
                is_pallet_installed::<pallet_timestamp::Pallet::<$runtime>>(),
                "pallet_timestamp is not installed in {runtime_name}"
            );
        }
    };
}
