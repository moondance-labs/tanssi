use snowbridge_beacon_primitives::{
    types::deneb, BeaconHeader, ExecutionProof, VersionedExecutionPayloadHeader,
};

pub fn mock_execution_proof() -> ExecutionProof {
    ExecutionProof {
        header: BeaconHeader::default(),
        ancestry_proof: None,
        execution_header: VersionedExecutionPayloadHeader::Deneb(deneb::ExecutionPayloadHeader {
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
        }),
        execution_branch: vec![],
    }
}
