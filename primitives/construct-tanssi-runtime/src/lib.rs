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
    pub use impls::impls;
}

pub trait Config {
    const SLOT_DURATION: u64;
}

#[macro_export]
macro_rules! construct_tanssi_runtime {
    (
        pub enum $runtime:ident $($inner:tt)+
    ) => {

        const _:() = {
            use $crate::deps::*;

            impl pallet_author_inherent::Config for $runtime {
                type AuthorId = NimbusId;
                type AccountLookup = tp_consensus::NimbusLookUp;
                type CanAuthor = pallet_cc_authorities_noting::CanAuthor<$runtime>;
                type SlotBeacon = tp_consensus::AuraDigestSlotBeacon<$runtime>;
                type WeightInfo = pallet_author_inherent::weights::SubstrateWeight<$runtime>;
            }
        };

        construct_runtime!(
            pub enum $runtime $($inner)+
        );

        #[test]
        fn __construct_tanssi_runtime_tests() {
            use $crate::deps::impls;

            let runtime_name = stringify!($runtime);

            assert!(impls!($runtime: $crate::Config), "{runtime_name} must impl tp_construct_tanssi_runtime::Config");
            assert!(impls!(RuntimeError: From<pallet_author_inherent::Error<$runtime>>), "pallet_author_inherent is not installed in {runtime_name}");
        }
    };
}
