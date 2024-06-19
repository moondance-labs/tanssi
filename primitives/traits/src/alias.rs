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

#[doc(hidden)]
pub mod __reexports {
    pub use {
        frame_support::{CloneNoBound, EqNoBound, PartialEqNoBound, RuntimeDebugNoBound},
        scale_info::TypeInfo,
        sp_core::{Decode, Encode, RuntimeDebug},
    };
}

pub use macro_rules_attribute::apply;

#[macro_export]
macro_rules! derive_storage_traits {
    ( $( $tt:tt )* ) => {
        #[derive(
            $crate::alias::__reexports::RuntimeDebug,
            ::core::cmp::PartialEq,
            ::core::cmp::Eq,
            ::core::clone::Clone,
            $crate::alias::__reexports::Encode,
            $crate::alias::__reexports::Decode,
            $crate::alias::__reexports::TypeInfo,
        )]
        $($tt)*
    }
}

// This currently doesn't work due to a quirk in RuntimeDebugNoBound, PartialEqNoBound
// and CloneNoBound, as there seem to be something breaking macro hygiene. This is not an
// issue when using the derive directly, but doesn't compile when adding it through our macro.
// #[macro_export]
// macro_rules! derive_storage_traits_no_bounds {
//     ( $( $tt:tt )* ) => (
//         #[derive(
//             $crate::alias::__reexports::RuntimeDebugNoBound,
//             $crate::alias::__reexports::PartialEqNoBound,
//             $crate::alias::__reexports::EqNoBound,
//             $crate::alias::__reexports::CloneNoBound,
//             $crate::alias::__reexports::Encode,
//             $crate::alias::__reexports::Decode,
//             $crate::alias::__reexports::TypeInfo,
//         )]
//         $($tt)*
//     );
// }

/// Derives traits related to SCALE encoding and serde.
#[macro_export]
macro_rules! derive_scale_codec {
    ( $( $tt:tt )* ) => {
        #[derive(
            $crate::alias::__reexports::Encode,
            $crate::alias::__reexports::Decode,
            $crate::alias::__reexports::TypeInfo,
        )]
        $($tt)*
    }
}

macro_rules! trait_alias {
    ($vis:vis $alias:ident : $bound0:path $(, $boundN:path)* $(,)?) => {
        $vis trait $alias: $bound0 $(+ $boundN)* { }
        impl<T: $bound0 $(+ $boundN)*> $alias for T { }
    }
}

trait_alias!(pub ScaleCodec:
    __reexports::Encode,
    __reexports::Decode,
    __reexports::TypeInfo,
);

trait_alias!(pub StorageTraits:
    ::core::fmt::Debug,
    ::core::clone::Clone,
    ::core::cmp::Eq,
    ::core::cmp::PartialEq,
    ScaleCodec,
);
