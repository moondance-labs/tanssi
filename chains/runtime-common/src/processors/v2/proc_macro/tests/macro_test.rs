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

use snowbridge_inbound_queue_primitives::v2::*;
use sp_runtime::AccountId32;
use sp_runtime::{DispatchError, Weight};
use tanssi_runtime_common::processors::v2::*;
use v2_processor_proc_macro::MessageProcessor as MessageProcessorDerive;

/// This test checks semantics of generated snowbridge MessageProcessor trait.
///
/// How this test works?
/// There are two test implementation of `MessageProcessorWithFallback` with two test implementation of
/// `FallbackMessageProcessor` integrated with each.
///
/// For each MessageProcessorWithFallback:
/// 1. `try_process_message`: Returns either Ok, InvalidMessage or UnsupportedMessage based on message.value
/// 2. `process_extracted_message`: Returns Ok for message.value for which `try_process_message` also return Ok, Return Err otherwise
///
/// For each FallbackMessageProcessor:
/// 1. `handle_message`: Returns Err for specific value of message.execution_fee, Ok otherwise
///
/// To test macro implementation semantics, we define test table for each method implemented by macro (For both implementation individually and combined using tuple implementation):
/// 1. `can_process_message`: Each row in table consists of two elements, first element contains the message.value and second element consist of boolean expected return value
/// 2. `process_message`: Each row in table consists of three element, the first two elements contain message.value and message.execution_fee respectively and the last element contains expected return value
///
/// The test will pass when the macro implementation behaves consistently as per tables.
///

struct TestFallback1;

impl<AccountId> FallbackMessageProcessor<AccountId> for TestFallback1 {
    fn handle_message(
        _who: AccountId,
        message: Message,
    ) -> Result<Option<Weight>, MessageProcessorError> {
        match message.execution_fee {
            1 => Err(MessageProcessorError::ProcessMessage(DispatchError::Other(
                "TestFallback1ProcessorError",
            ))),
            _ => Ok(None),
        }
    }
}

#[derive(MessageProcessorDerive)]
struct TestImpl1;

impl<AccountId> MessageProcessorWithFallback<AccountId> for TestImpl1
where
    AccountId: From<[u8; 32]>,
{
    type Fallback = TestFallback1;
    type ExtractedMessage = u128;

    fn try_extract_message(
        _who: &AccountId,
        message: &Message,
    ) -> Result<Self::ExtractedMessage, MessageExtractionError> {
        match message.value {
            1 => Err(MessageExtractionError::InvalidMessage {
                context: "TestImpl1ExtractionError1".to_string(),
                source: None,
            }),
            2 => Ok(message.value),
            3 | 4 => Err(MessageExtractionError::UnsupportedMessage {
                context: "TestImpl1ExtractionError34".to_string(),
                source: None,
            }),
            _ => Err(MessageExtractionError::Other {
                context: "TestImpl1ExtractionError.".to_string(),
                source: None,
            }),
        }
    }

    fn process_extracted_message(
        _sender: AccountId,
        extracted_message: Self::ExtractedMessage,
    ) -> Result<Option<Weight>, MessageProcessorError> {
        match extracted_message {
            2 => Ok(None),
            _ => Err(MessageProcessorError::ProcessMessage(DispatchError::Other(
                "TestImpl1MainProcessorError",
            ))),
        }
    }

    fn worst_case_message_processor_weight() -> Weight {
        Weight::zero()
    }

    fn calculate_message_id(message: &Message) -> [u8; 32] {
        let response = Self::try_extract_message(&AccountId::from([1u8; 32]), message);
        match response {
            Ok(value) => [value as u8; 32],
            Err(MessageExtractionError::UnsupportedMessage { .. }) => [0; 32],
            Err(MessageExtractionError::InvalidMessage { .. }) => {
                [(message.execution_fee + 1) as u8; 32]
            }
            _ => [0u8; 32],
        }
    }
}

struct TestFallback2;

impl<AccountId> FallbackMessageProcessor<AccountId> for TestFallback2 {
    fn handle_message(
        _who: AccountId,
        message: Message,
    ) -> Result<Option<Weight>, MessageProcessorError> {
        match message.execution_fee {
            2 => Err(MessageProcessorError::ProcessMessage(DispatchError::Other(
                "TestFallback1ProcessorError",
            ))),
            _ => Ok(None),
        }
    }
}

#[derive(MessageProcessorDerive)]
struct TestImpl2;

impl<AccountId> MessageProcessorWithFallback<AccountId> for TestImpl2
where
    AccountId: From<[u8; 32]>,
{
    type Fallback = TestFallback2;
    type ExtractedMessage = u128;

    fn try_extract_message(
        _sender: &AccountId,
        message: &Message,
    ) -> Result<Self::ExtractedMessage, MessageExtractionError> {
        match message.value {
            1 | 2 => Err(MessageExtractionError::UnsupportedMessage {
                context: "TestImpl2ExtractionError12".to_string(),
                source: None,
            }),
            3 => Err(MessageExtractionError::InvalidMessage {
                context: "TestImpl2ExtractionError3".to_string(),
                source: None,
            }),
            4 => Ok(message.value),
            _ => Err(MessageExtractionError::Other {
                context: "TestImpl2ExtractionError.".to_string(),
                source: None,
            }),
        }
    }

    fn process_extracted_message(
        _sender: AccountId,
        extracted_message: Self::ExtractedMessage,
    ) -> Result<Option<Weight>, MessageProcessorError> {
        match extracted_message {
            4 => Ok(None),
            _ => Err(MessageProcessorError::ProcessMessage(DispatchError::Other(
                "TestImpl2MainProcessorError",
            ))),
        }
    }

    fn worst_case_message_processor_weight() -> Weight {
        Weight::zero()
    }

    fn calculate_message_id(message: &Message) -> [u8; 32] {
        let response = Self::try_extract_message(&AccountId::from([1u8; 32]), message);
        match response {
            Ok(value) => [value as u8; 32],
            Err(MessageExtractionError::UnsupportedMessage { .. }) => [0; 32],
            Err(MessageExtractionError::InvalidMessage { .. }) => [(message.value + 1) as u8; 32],
            _ => [0u8; 32],
        }
    }
}

type Processors = (TestImpl1, TestImpl2);

#[test]
fn test_macro_can_process_semantics() {
    let can_process_table_1 = [true, true, true, false, false, true];
    let can_process_table_2 = [true, false, false, true, true, true];

    let combined_can_process_table = can_process_table_1
        .iter()
        .zip(can_process_table_2)
        .map(|(impl_1_table, impl_2_table)| *impl_1_table || impl_2_table)
        .collect::<Vec<bool>>();

    for (eth_value, can_process) in can_process_table_1.iter().enumerate() {
        let message = Message {
            gateway: Default::default(),
            nonce: 0,
            origin: Default::default(),
            assets: vec![],
            payload: Payload::Raw(Vec::new()),
            claimer: None,
            value: eth_value as u128,
            execution_fee: 0,
            relayer_fee: 0,
        };

        let response = TestImpl1::can_process_message(&AccountId32::new([0; 32]), &message);

        assert_eq!(
            *can_process, response,
            "TestImpl1 expected response {} for value {} but found {}",
            *can_process, eth_value, response
        );
    }

    for (eth_value, can_process) in can_process_table_2.iter().enumerate() {
        let message = Message {
            gateway: Default::default(),
            nonce: 0,
            origin: Default::default(),
            assets: vec![],
            payload: Payload::Raw(Vec::new()),
            claimer: None,
            value: eth_value as u128,
            execution_fee: 0,
            relayer_fee: 0,
        };

        let response = TestImpl2::can_process_message(&AccountId32::new([0; 32]), &message);

        assert_eq!(
            *can_process, response,
            "TestImpl2 expected response {} for value {} but found {}",
            *can_process, eth_value, response
        );
    }

    for (eth_value, can_process) in combined_can_process_table.iter().enumerate() {
        let message = Message {
            gateway: Default::default(),
            nonce: 0,
            origin: Default::default(),
            assets: vec![],
            payload: Payload::Raw(Vec::new()),
            claimer: None,
            value: eth_value as u128,
            execution_fee: 0,
            relayer_fee: 0,
        };

        let response = Processors::can_process_message(&AccountId32::new([0; 32]), &message);

        assert_eq!(
            *can_process, response,
            "For Combined processors expected response {} for value {} but found {}",
            *can_process, eth_value, response
        );
    }
}

#[test]
fn test_macro_process_message_semantics() {
    let process_message_table_1 = [
        (
            0,
            0,
            Err(MessageProcessorError::ProcessMessage(DispatchError::Other(
                "Other error while processing v2 message",
            ))),
        ),
        (
            0,
            1,
            Err(MessageProcessorError::ProcessMessage(DispatchError::Other(
                "Other error while processing v2 message",
            ))),
        ),
        (
            0,
            2,
            Err(MessageProcessorError::ProcessMessage(DispatchError::Other(
                "Other error while processing v2 message",
            ))),
        ),
        (1, 0, Ok(([1u8; 32], None))),
        (
            1,
            1,
            Err(MessageProcessorError::ProcessMessage(DispatchError::Other(
                "TestFallback1ProcessorError",
            ))),
        ),
        (1, 2, Ok(([3u8; 32], None))),
        (2, 0, Ok(([2u8; 32], None))),
        (2, 1, Ok(([2u8; 32], None))),
        (2, 2, Ok(([2u8; 32], None))),
        (
            3,
            0,
            Err(MessageProcessorError::ProcessMessage(DispatchError::Other(
                "Unsupported v2 message",
            ))),
        ),
        (
            3,
            1,
            Err(MessageProcessorError::ProcessMessage(DispatchError::Other(
                "Unsupported v2 message",
            ))),
        ),
        (
            3,
            2,
            Err(MessageProcessorError::ProcessMessage(DispatchError::Other(
                "Unsupported v2 message",
            ))),
        ),
        (
            4,
            0,
            Err(MessageProcessorError::ProcessMessage(DispatchError::Other(
                "Unsupported v2 message",
            ))),
        ),
        (
            4,
            1,
            Err(MessageProcessorError::ProcessMessage(DispatchError::Other(
                "Unsupported v2 message",
            ))),
        ),
        (
            4,
            2,
            Err(MessageProcessorError::ProcessMessage(DispatchError::Other(
                "Unsupported v2 message",
            ))),
        ),
        (
            5,
            0,
            Err(MessageProcessorError::ProcessMessage(DispatchError::Other(
                "Other error while processing v2 message",
            ))),
        ),
        (
            5,
            1,
            Err(MessageProcessorError::ProcessMessage(DispatchError::Other(
                "Other error while processing v2 message",
            ))),
        ),
        (
            5,
            2,
            Err(MessageProcessorError::ProcessMessage(DispatchError::Other(
                "Other error while processing v2 message",
            ))),
        ),
    ];

    for (eth_value, execution_fee, expected_response) in &process_message_table_1 {
        let message = Message {
            gateway: Default::default(),
            nonce: 0,
            origin: Default::default(),
            assets: vec![],
            payload: Payload::Raw(Vec::new()),
            claimer: None,
            value: *eth_value,
            execution_fee: *execution_fee,
            relayer_fee: 0,
        };

        let response = TestImpl1::process_message(AccountId32::new([0; 32]), message);

        assert_eq!(
            response, *expected_response,
            "For TestImpl1 expected response {:?} for value ({}, {}) but found {:?}",
            expected_response, eth_value, execution_fee, response
        );
    }

    let process_message_table_2 = [
        (
            0,
            0,
            Err(MessageProcessorError::ProcessMessage(DispatchError::Other(
                "Other error while processing v2 message",
            ))),
        ),
        (
            0,
            1,
            Err(MessageProcessorError::ProcessMessage(DispatchError::Other(
                "Other error while processing v2 message",
            ))),
        ),
        (
            0,
            2,
            Err(MessageProcessorError::ProcessMessage(DispatchError::Other(
                "Other error while processing v2 message",
            ))),
        ),
        (
            1,
            0,
            Err(MessageProcessorError::ProcessMessage(DispatchError::Other(
                "Unsupported v2 message",
            ))),
        ),
        (
            1,
            1,
            Err(MessageProcessorError::ProcessMessage(DispatchError::Other(
                "Unsupported v2 message",
            ))),
        ),
        (
            1,
            2,
            Err(MessageProcessorError::ProcessMessage(DispatchError::Other(
                "Unsupported v2 message",
            ))),
        ),
        (
            2,
            0,
            Err(MessageProcessorError::ProcessMessage(DispatchError::Other(
                "Unsupported v2 message",
            ))),
        ),
        (
            2,
            1,
            Err(MessageProcessorError::ProcessMessage(DispatchError::Other(
                "Unsupported v2 message",
            ))),
        ),
        (
            2,
            2,
            Err(MessageProcessorError::ProcessMessage(DispatchError::Other(
                "Unsupported v2 message",
            ))),
        ),
        (3, 0, Ok(([4u8; 32], None))),
        (3, 1, Ok(([4u8; 32], None))),
        (
            3,
            2,
            Err(MessageProcessorError::ProcessMessage(DispatchError::Other(
                "TestFallback1ProcessorError",
            ))),
        ),
        (4, 0, Ok(([4u8; 32], None))),
        (4, 1, Ok(([4u8; 32], None))),
        (4, 2, Ok(([4u8; 32], None))),
        (
            5,
            0,
            Err(MessageProcessorError::ProcessMessage(DispatchError::Other(
                "Other error while processing v2 message",
            ))),
        ),
        (
            5,
            1,
            Err(MessageProcessorError::ProcessMessage(DispatchError::Other(
                "Other error while processing v2 message",
            ))),
        ),
        (
            5,
            2,
            Err(MessageProcessorError::ProcessMessage(DispatchError::Other(
                "Other error while processing v2 message",
            ))),
        ),
    ];

    for (eth_value, execution_fee, expected_response) in &process_message_table_2 {
        let message = Message {
            gateway: Default::default(),
            nonce: 0,
            origin: Default::default(),
            assets: vec![],
            payload: Payload::Raw(Vec::new()),
            claimer: None,
            value: *eth_value,
            execution_fee: *execution_fee,
            relayer_fee: 0,
        };

        let response = TestImpl2::process_message(AccountId32::new([0; 32]), message);

        assert_eq!(
            response, *expected_response,
            "For TestImpl2 expected response {:?} for value ({}, {}) but found {:?}",
            expected_response, eth_value, execution_fee, response
        );
    }

    let combined_process_message_table = process_message_table_1
        .iter()
        .zip(process_message_table_2)
        .map(|(impl_1_table, impl_2_table)| {
            assert_eq!(impl_1_table.0, impl_2_table.0);
            assert_eq!(impl_1_table.1, impl_2_table.1);

            let combined_result = if impl_1_table.2.is_ok() {
                impl_1_table.2.clone()
            } else {
                match &impl_1_table.2 {
                    Err(MessageProcessorError::ProcessMessage(DispatchError::Other(error))) => {
                        if error.contains("Unsupported") {
                            match &impl_2_table.2 {
                                Ok(_) => impl_2_table.2.clone(),
                                Err(MessageProcessorError::ProcessMessage(
                                    DispatchError::Other(error),
                                )) => {
                                    if error.contains("Unsupported Message") {
                                        Err(MessageProcessorError::ProcessMessage(
                                            DispatchError::Other("No handler found for message!"),
                                        ))
                                    } else {
                                        impl_2_table.2
                                    }
                                }
                                _ => panic!("Unexpected error type"),
                            }
                        } else {
                            impl_1_table.2.clone()
                        }
                    }
                    _ => panic!("Unexpected error type"),
                }
            };

            (impl_2_table.0, impl_2_table.1, combined_result)
        })
        .collect::<Vec<_>>();

    for (eth_value, execution_fee, expected_response) in &combined_process_message_table {
        let message = Message {
            gateway: Default::default(),
            nonce: 0,
            origin: Default::default(),
            assets: vec![],
            payload: Payload::Raw(Vec::new()),
            claimer: None,
            value: *eth_value,
            execution_fee: *execution_fee,
            relayer_fee: 0,
        };

        let response = Processors::process_message(AccountId32::new([0; 32]), message);

        assert_eq!(
            response, *expected_response,
            "For Combined processors expected response {:?} for value ({}, {}) but found {:?}",
            expected_response, eth_value, execution_fee, response
        );
    }
}
