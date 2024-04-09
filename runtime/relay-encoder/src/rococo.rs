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

use {cumulus_primitives_core::ParaId, parity_scale_codec::Encode};

pub type Balance = u128;

#[derive(Encode)]
pub enum RelayCall {
    #[codec(index = 66u8)]
    OnDemandAssignmentProvider(OnDemandAssignmentProviderCall),
}

#[derive(Encode)]
pub enum OnDemandAssignmentProviderCall {
    #[codec(index = 0u8)]
    PlaceOrderAllowDeath {
        max_amount: Balance,
        para_id: ParaId,
    },
}

#[cfg(test)]
mod tests {
    use {
        super::*, polkadot_runtime_parachains::assigner_on_demand as parachains_assigner_on_demand,
    };

    #[test]
    fn encode_place_order_allow_death() {
        let max_amount = u128::MAX;
        let para_id = u32::MAX.into();
        let call = rococo_runtime::RuntimeCall::OnDemandAssignmentProvider(
            parachains_assigner_on_demand::Call::place_order_allow_death {
                max_amount,
                para_id,
            },
        );
        let call2 = RelayCall::OnDemandAssignmentProvider(
            OnDemandAssignmentProviderCall::PlaceOrderAllowDeath {
                max_amount,
                para_id,
            },
        );

        // If this fails check most probably indices changed
        assert_eq!(call.encode(), call2.encode());
    }
}
