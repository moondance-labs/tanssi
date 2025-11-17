use snowbridge_inbound_queue_primitives::v2::*;
use sp_runtime::AccountId32;
use sp_runtime::DispatchError;
use tanssi_runtime_common::processors::v2::*;
use v2_processor_proc_macro::MessageProcessor as MessageProcessorDerive;

struct TestFallback1;

impl<AccountId> FallbackMessageProcessor<AccountId> for TestFallback1 {
    fn handle_message(
        _who: AccountId,
        message: Message,
    ) -> Result<[u8; 32], MessageProcessorError> {
        match message.execution_fee {
            1 => Err(MessageProcessorError::ProcessMessage(DispatchError::Other(
                "TestFallback1ProcessorError",
            ))),
            _ => Ok([1u8; 32]),
        }
    }
}

#[derive(MessageProcessorDerive)]
struct TestImpl1;

impl<AccountId> MessageProcessorWithFallback<AccountId> for TestImpl1 {
    type Fallback = TestFallback1;
    type ExtractedMessage = u128;

    fn try_extract_message(
        _who: &AccountId,
        message: &Message,
    ) -> Result<Self::ExtractedMessage, MessageExtractionError> {
        match message.value {
            1 => Err(MessageExtractionError::InvalidMessage {
                context: "TestImpl1ExtractionError2".to_string(),
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
    ) -> Result<[u8; 32], MessageProcessorError> {
        match extracted_message {
            2 => Ok([0u8; 32]),
            _ => Err(MessageProcessorError::ProcessMessage(DispatchError::Other(
                "TestImpl1MainProcessorError",
            ))),
        }
    }
}

struct TestFallback2;

impl<AccountId> FallbackMessageProcessor<AccountId> for TestFallback2 {
    fn handle_message(
        _who: AccountId,
        message: Message,
    ) -> Result<[u8; 32], MessageProcessorError> {
        match message.execution_fee {
            2 => Err(MessageProcessorError::ProcessMessage(DispatchError::Other(
                "TestFallback1ProcessorError",
            ))),
            _ => Ok([1u8; 32]),
        }
    }
}

#[derive(MessageProcessorDerive)]
struct TestImpl2;

impl<AccountId> MessageProcessorWithFallback<AccountId> for TestImpl2 {
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
    ) -> Result<[u8; 32], MessageProcessorError> {
        match extracted_message {
            4 => Ok([0u8; 32]),
            _ => Err(MessageProcessorError::ProcessMessage(DispatchError::Other(
                "TestImpl2MainProcessorError",
            ))),
        }
    }
}

type Processors = (TestImpl1, TestImpl2);

#[test]
fn test_macro_can_process_semantics() {
    let can_process_table_1 = [false, true, true, false, false, false];
    let can_process_table_2 = [false, false, false, true, true, false];

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
        (0, 0, false),
        (0, 1, false),
        (0, 2, false),
        (1, 0, true),
        (1, 1, false),
        (1, 2, true),
        (2, 0, true),
        (2, 1, true),
        (2, 2, true),
        (3, 0, false),
        (3, 1, false),
        (3, 2, false),
        (4, 0, false),
        (4, 1, false),
        (4, 2, false),
        (5, 0, false),
        (5, 1, false),
        (5, 2, false),
    ];

    for (eth_value, execution_fee, is_ok) in process_message_table_1 {
        let message = Message {
            gateway: Default::default(),
            nonce: 0,
            origin: Default::default(),
            assets: vec![],
            payload: Payload::Raw(Vec::new()),
            claimer: None,
            value: eth_value,
            execution_fee,
            relayer_fee: 0,
        };

        assert_eq!(
            TestImpl1::process_message(AccountId32::new([0; 32]), message).is_ok(),
            is_ok,
            "For TestImpl1 expected is_ok: {} for value ({}, {}) but found {}",
            is_ok,
            eth_value,
            execution_fee,
            !is_ok
        );
    }

    let process_message_table_2 = [
        (0, 0, false),
        (0, 1, false),
        (0, 2, false),
        (1, 0, false),
        (1, 1, false),
        (1, 2, false),
        (2, 0, false),
        (2, 1, false),
        (2, 2, false),
        (3, 0, true),
        (3, 1, true),
        (3, 2, false),
        (4, 0, true),
        (4, 1, true),
        (4, 2, true),
        (5, 0, false),
        (5, 1, false),
        (5, 2, false),
    ];

    for (eth_value, execution_fee, is_ok) in process_message_table_2 {
        let message = Message {
            gateway: Default::default(),
            nonce: 0,
            origin: Default::default(),
            assets: vec![],
            payload: Payload::Raw(Vec::new()),
            claimer: None,
            value: eth_value,
            execution_fee,
            relayer_fee: 0,
        };

        assert_eq!(
            TestImpl2::process_message(AccountId32::new([0; 32]), message).is_ok(),
            is_ok,
            "For TestImpl2 expected is_ok: {} for value ({}, {}) but found {}",
            is_ok,
            eth_value,
            execution_fee,
            !is_ok
        );
    }

    let combined_process_message_table = process_message_table_1
        .iter()
        .zip(process_message_table_2)
        .map(|(impl_1_table, impl_2_table)| {
            assert_eq!(impl_1_table.0, impl_2_table.0);
            assert_eq!(impl_1_table.1, impl_2_table.1);
            (
                impl_2_table.0,
                impl_2_table.1,
                impl_1_table.2 || impl_2_table.2,
            )
        });

    for (eth_value, execution_fee, is_ok) in combined_process_message_table {
        let message = Message {
            gateway: Default::default(),
            nonce: 0,
            origin: Default::default(),
            assets: vec![],
            payload: Payload::Raw(Vec::new()),
            claimer: None,
            value: eth_value,
            execution_fee,
            relayer_fee: 0,
        };

        assert_eq!(
            Processors::process_message(AccountId32::new([0; 32]), message).is_ok(),
            is_ok,
            "For Combined processor expected is_ok: {} for value ({}, {}) but found {}",
            is_ok,
            eth_value,
            execution_fee,
            !is_ok
        );
    }
}
