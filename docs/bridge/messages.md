# Introduction
This document outlines how the messaging works between starlight and ethereum. It will describe each type of message supported by either side and then 
will describe the general flow of the messages from creation to handling of it.

# Commits of both tanssi-symbiotic and dancelight runtime referenced
- Tanssi-symbiotic: https://github.com/moondance-labs/tanssi-symbiotic/commit/1428c2a436a54edb154274bebc94daf390ecf125
- Tanssi: https://github.com/moondance-labs/tanssi/commit/ac9c25b0e3f2a45a7def57b556b8b666d7949a3f

# ReceiveValidators
Sends account ids of operators from symbiotic to starlight.

## Direction
Ethereum -> Starlight

## Payload
```solidity
return bytes.concat(
    bytes4(0x70150038), // Magic bytes
    bytes1(uint8(Message.V0)), // Message versioning bytes
    bytes1(uint8(OutboundCommandV1.ReceiveValidators)), // Enum byte
    ScaleCodec.encodeCompactU32(operatorsCount), // Operator count
    operatorsFlattened, // Operators array
    ScaleCodec.encodeU64(uint64(epoch)) // timestamp
);
```

```rust
/// Payload is the whole data we expect to receive from the relayer
#[derive(Encode, Decode)]
pub struct Payload<T>
where
    T: pallet_external_validators::Config,
{
    pub magic_bytes: [u8; 4],
    pub message: Message<T>,
}

/// Actual message inside the payload
#[derive(Encode, Decode)]
pub enum Message<T>
where
    T: pallet_external_validators::Config,
{
    V1(InboundCommand<T>),
}

/// Command to be executed by this message processor
#[derive(Encode, Decode)]
pub enum InboundCommand<T>
where
    T: pallet_external_validators::Config,
{
    ReceiveValidators {
        validators: Vec<<T as pallet_external_validators::Config>::ValidatorId>,
        timestamp: u64,
    },
}
```

## Where and when the payload is created?
This payload is created on ethereum side to send operators data to starlight. The timestamp here is an opaque type created by ethereum and not semantically interpreted by starlight.

## Where this payload is consumed?
The payload is consumed by `external-validators` and the validator ids contained in the message are set as external validators. The message processor is `SymbioticMessageProcessor` which calls `ExternalRewards::set_external_validators`.

# ReportRewards
It is used to report rewards of operators producing blocks on starlight.

## Direction
Sarlight -> Ethereum

## Payload
```rust
pub enum Command {
    /// snippet
    ReportRewards {
        // block timestamp
        timestamp: u64,
        // index of the era we are sending info of
        era_index: u32,
        // total_points for the era
        total_points: u128,
        // new tokens inflated during the era
        tokens_inflated: u128,
        // merkle root of vec![(validatorId, rewardPoints)]
        rewards_merkle_root: H256,
    }
}
```

This payload is sent from starlight as outbound command. The timestamp here is originally the same timestamp sent from ethereum side as part of ReceiveValidators command. The `rewards_merkle_root` here refers to markle tree of reward data per validators.
These are the same validators sent by ethereum in ReceiveValidators command.

## Where and when the payload is created?
This payload is created in pallet external-validator-rewards in the `on_era_end` handler. This handler is triggered at the ending of an `era`.

## Where this payload is consumed?
This payload is consumed by our implementation of Symbiotic middleware when called by Gateway to handle rewards. It is consumed in function called `distributeRewards` in the middleware.

# ReportSlashes
It is used to report slashing of operators producing blocks on starlight.

## Direction
Starlight -> Ethereum

## Payload
```rust
/// A command which is executable by the Gateway contract on Ethereum
#[derive(Clone, Encode, Decode, RuntimeDebug, TypeInfo, PartialEq)]
pub enum Command {
    /// snippet
    ReportSlashes {
        // index of the era we are sending info of
        era_index: u32,
        // vec of `SlashData`
        slashes: Vec<SlashData>,
    },
}
```

This payload is sent from starlight as outbound command. The era_index here is starlight's era index in which the slashing has occurred. The ethereum side maintains a mapping from starlight era index to ethereum timestamp. This allows ethereum side to deduce for which epoch we need to slash a validator. (This will be changed in future.)

## Where and when the payload is created?
The payload is created in `external-validator-slashes` pallet in `process_slashes_queue` and handed off to outbound queue pallet. 

## Where this payload is consumed?
Each slash is handled by Middleware in a function called `slash`.


