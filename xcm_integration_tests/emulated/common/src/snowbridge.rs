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

use {
    snowbridge_beacon_primitives::{types::deneb, ExecutionProof, VersionedExecutionPayloadHeader},
    snowbridge_core::inbound::Proof,
};

pub fn mock_snowbridge_message_proof() -> Proof {
    Proof {
        receipt_proof: (vec![], vec![]),
        execution_proof: ExecutionProof {
            header: Default::default(),
            ancestry_proof: None,
            execution_header: VersionedExecutionPayloadHeader::Deneb(
                deneb::ExecutionPayloadHeader {
                    parent_hash: Default::default(),
                    fee_recipient: Default::default(),
                    state_root: Default::default(),
                    receipts_root: Default::default(),
                    logs_bloom: vec![],
                    prev_randao: Default::default(),
                    block_number: 0,
                    gas_limit: 0,
                    gas_used: 0,
                    timestamp: 0,
                    extra_data: vec![],
                    base_fee_per_gas: Default::default(),
                    block_hash: Default::default(),
                    transactions_root: Default::default(),
                    withdrawals_root: Default::default(),
                    blob_gas_used: 0,
                    excess_blob_gas: 0,
                },
            ),
            execution_branch: vec![],
        },
    }
}
