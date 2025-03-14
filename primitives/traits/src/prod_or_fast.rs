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
    pub use frame_support::pallet_prelude::Get;
}

/// Define a `Get` which can have different value in "prod" or "fast" mode.
/// `Get` will return "fast" value if feature "fast-runtime" is enabled.
/// It also provides functions `prod`/`prod_if`/`fast` to write tests for both
/// values, regardless of the feature enabled when compiling the tests.
#[macro_export]
macro_rules! prod_or_fast_parameter_types {
    ($( #[ $attr:meta ] )* $vis:vis const $name:ident: $ty:ty = { prod: $prod:expr, fast: $fast:expr }; $($rest:tt)* ) => {
        $( #[ $attr ] )*
        $vis struct $name;

        impl $name {
            /// Get the value based on if "fast-runtime" feature is enabled.
            pub const fn get() -> $ty {
                Self::prod_if(!cfg!(feature = "fast-runtime"))
            }

            /// Return prod value if condition is true, otherwise returns fast value.
            pub const fn prod_if(b: bool) -> $ty {
                if b { Self::prod() } else { Self::fast() }
            }

            /// Always return prod value.
            pub const fn prod() -> $ty {
                $prod
            }

            /// Always return fast value.
            pub const fn fast() -> $ty {
                $fast
            }
        }

        impl<_I: From<$ty>> $crate::prod_or_fast::__reexports::Get<_I> for $name {
            fn get() -> _I {
                _I::from(Self::get())
            }
        }

        $crate::prod_or_fast_parameter_types!($($rest)*);
    };
    ($( #[ $attr:meta ] )* $vis:vis $name:ident: $ty:ty = { prod: $prod:expr, fast: $fast:expr }; $($rest:tt)* ) => {
        $( #[ $attr ] )*
        $vis struct $name;

        impl $name {
            /// Get the value based on if "fast-runtime" feature is enabled.
            pub fn get() -> $ty {
                Self::prod_if(!cfg!(feature = "fast-runtime"))
            }

            /// Return prod value if condition is true, otherwise returns fast value.
            pub fn prod_if(b: bool) -> $ty {
                if b { Self::prod() } else { Self::fast() }
            }

            /// Always return prod value.
            pub fn prod() -> $ty {
                $prod
            }

            /// Always return fast value.
            pub fn fast() -> $ty {
                $fast
            }
        }

        impl<_I: From<$ty>> $crate::prod_or_fast::__reexports::Get<_I> for $name {
            fn get() -> _I {
                _I::from(Self::get())
            }
        }

        $crate::prod_or_fast_parameter_types!($($rest)*);
    };
    () => {}
}
