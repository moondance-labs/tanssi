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

//! Runtime API for Services Payment pallet

#![cfg_attr(not(feature = "std"), no_std)]

sp_api::decl_runtime_apis! {
    pub trait ServicesPaymentApi<Balance, ParaId>
    where
        Balance: parity_scale_codec::Codec,
        ParaId: parity_scale_codec::Codec,
    {
        fn block_cost(para_id: ParaId) -> Balance;
        fn collator_assignment_cost(para_id: ParaId) -> Balance;
    }
}
