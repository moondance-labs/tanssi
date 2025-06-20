# Introduction
This document outlines how the messaging works between starlight and ethereum. It will describe each type of message supported by either side and then 
will describe the general flow of the messages from creation to handling of it.

# Commits of both tanssi-symbiotic and dancelight runtime referenced
TODO: outdated
- Tanssi-symbiotic: https://github.com/moondance-labs/tanssi-symbiotic/commit/1428c2a436a54edb154274bebc94daf390ecf125
- Tanssi: https://github.com/moondance-labs/tanssi/commit/ac9c25b0e3f2a45a7def57b556b8b666d7949a3f

# Ethereum -> Starlight (starlight processor)

The starlight processor handles messages unique to starlight, not present in snowbridge.

Ethereum sees this commands as outbound, and starlight sees the command as inbound.
So it is confusing but OutboundCommandV1 in ethereum == InboundCommand in starlight

[OutboundCommandV1](https://github.com/moondance-labs/tanssi-bridge-relayer/blob/092097156958c11a5929a538fbbf5e29f9388762/overridden_contracts/src/libraries/OSubstrateTypes.sol#L26)

```solidity
enum OutboundCommandV1 {
    ReceiveValidators
}
```

[InboundCommand](https://github.com/moondance-labs/tanssi/blob/3a1e91756e1b5f6cfd40590db9ee4e133fc4a8b0/primitives/bridge/src/symbiotic_message_processor.rs#L54)

```rust
pub enum InboundCommand<T> {
    ReceiveValidators {
        validators: Vec<<T as pallet_external_validators::Config>::ValidatorId>,
        external_index: u64,
    },
}
```

This is an enum with custom commands that are routed to a starlight processor, not the snowbridge processor.
For now, only used to receive the external validators list.

## ReceiveValidators
Sends account ids of operators from symbiotic to starlight.

### Who can send it and when

[Gateway.sendOperatorsData](https://github.com/moondance-labs/tanssi-bridge-relayer/blob/8f563fb5af462eadf9881a1b00153170a2c8cb1a/overridden_contracts/src/Gateway.sol#L607)

onlyMiddleware

[Middleware.sendCurrentOperatorsKeys](https://github.com/moondance-labs/tanssi-symbiotic/blob/e81b715b4c4c0a1e121ecafcf85a789faea642d2/src/contracts/middleware/Middleware.sol#L282C14-L282C38)

external, anyone can call it

[MIN_INTERVAL_TO_SEND_OPERATOR_KEYS](https://github.com/moondance-labs/tanssi-symbiotic/blob/e81b715b4c4c0a1e121ecafcf85a789faea642d2/src/contracts/middleware/MiddlewareStorage.sol#L36)

minimum period of 10 minutes

can be called more than once for the same epoch, will send a message to starlight but the operators will be the same

Can be called by chainlink nodes without any on-chain validations. But only once per epoch.

[Middleware.performUpkeep](https://github.com/moondance-labs/tanssi-symbiotic/blob/e81b715b4c4c0a1e121ecafcf85a789faea642d2/src/contracts/middleware/Middleware.sol#L338)

### Payload

Encoding on solidity side:

[OSubstrateTypes.EncodedOperatorsData](https://github.com/moondance-labs/tanssi-bridge-relayer/blob/092097156958c11a5929a538fbbf5e29f9388762/overridden_contracts/src/libraries/OSubstrateTypes.sol#L40)

```solidity
return bytes.concat(
    bytes4(0x70150038), // Magic bytes
    bytes1(uint8(Message.V0)), // Message versioning bytes, 0x00
    bytes1(uint8(OutboundCommandV1.ReceiveValidators)), // Enum byte, 0x00
    ScaleCodec.encodeCompactU32(operatorsCount), // Operator count
    operatorsFlattened, // Operators array
    ScaleCodec.encodeU64(uint64(epoch)) // timestamp
);
```

Validations on solidity side:

[Operators.encodeOperatorsData](https://github.com/moondance-labs/tanssi-bridge-relayer/blob/092097156958c11a5929a538fbbf5e29f9388762/overridden_contracts/src/Operators.sol#L35)

```solidity
if (operatorsKeys.length == 0) {
    revert Operators__OperatorsKeysCannotBeEmpty();
}
if (validatorsKeysLength > MAX_OPERATORS) { // MAX_OPERATORS = 1000
    revert Operators__OperatorsLengthTooLong();
}
```

Decoding on rust side:

[InboundCommand.ReceiveValidators](https://github.com/moondance-labs/tanssi/blob/3a1e91756e1b5f6cfd40590db9ee4e133fc4a8b0/primitives/bridge/src/symbiotic_message_processor.rs#L54)

Validations on rust side:

[process_message.process_message](https://github.com/moondance-labs/tanssi/blob/3a1e91756e1b5f6cfd40590db9ee4e133fc4a8b0/primitives/bridge/src/symbiotic_message_processor.rs#L105)

```rust
envelope.channel_id == PRIMARY_GOVERNANCE_CHANNEL
```

[ExternalValidators::set_external_validators_inner](https://github.com/moondance-labs/tanssi/blob/3a1e91756e1b5f6cfd40590db9ee4e133fc4a8b0/pallets/external-validators/src/lib.rs#L375)

No additional validations, truncates list to first 100 validators

[MaxExternalValidators](https://github.com/moondance-labs/tanssi/blob/3a1e91756e1b5f6cfd40590db9ee4e133fc4a8b0/chains/orchestrator-relays/runtime/starlight/src/lib.rs#L1571)

External index not used for setting validators, used for slashing:

[ExternalValidatorSlashes::on_era_start](https://github.com/moondance-labs/tanssi/blob/3a1e91756e1b5f6cfd40590db9ee4e133fc4a8b0/pallets/external-validator-slashes/src/lib.rs#L558)

## Where and when the payload is created?
This payload is created on ethereum side to send operators data to starlight. The timestamp here is an opaque type created by ethereum and not semantically interpreted by starlight.

## Where this payload is consumed?
The payload is consumed by `external-validators` and the validator ids contained in the message are set as external validators. The message processor is `SymbioticMessageProcessor` which calls `ExternalRewards::set_external_validators`.

# Ethereum -> Starlight (snowbridge processor)

[Snowbridge CallsV1](https://github.com/Snowfork/snowbridge/blob/50cfe0702793ab737dbe34f25994a05e7a22f921/contracts/src/v1/Calls.sol#L71)

Token transfers and registration. Both ERC20 tokens, starlight-native tokens, and ETH.

* registerToken

```solidity
SubstrateTypes.RegisterToken(token, $.assetHubCreateAssetFee)
```

* sendToken: 3 possible messages

```solidity
SubstrateTypes.SendTokenToAssetHubAddress32(
    token, destinationAddress.asAddress32(), $.assetHubReserveTransferFee, amount
);

SubstrateTypes.SendTokenToAddress32(
    token,
    destinationChain,
    destinationAddress.asAddress32(),
    $.assetHubReserveTransferFee,
    destinationChainFee,
    amount
)

SubstrateTypes.SendTokenToAddress20(
    token,
    destinationChain,
    destinationAddress.asAddress20(),
    $.assetHubReserveTransferFee,
    destinationChainFee,
    amount
)
```

## RegisterToken

Register an ERC20 token in Snowbrige contract. 

Anyone can call it, but they must pay a configurable `_registerTokenCosts()` fee.
The same token can be registered more than once, and it will always send a message to starlight.

### Payload

Encoding on solidity side:

[SubstrateTypes.RegisterToken](https://github.com/Snowfork/snowbridge/blob/50cfe0702793ab737dbe34f25994a05e7a22f921/contracts/src/SubstrateTypes.sol#L42)

```solidity
function RegisterToken(address token, uint128 fee) internal view returns (bytes memory) {
    return bytes.concat(
        bytes1(0x00), // Version byte
        ScaleCodec.encodeU64(uint64(block.chainid)), // ethereum chain id (0x01 for mainnet)
        bytes1(0x00), // Enum variant
        SubstrateTypes.H160(token),
        ScaleCodec.encodeU128(fee)
    );
}
```

Validations on solidity side:

[Functions.registerNativeToken](https://github.com/Snowfork/snowbridge/blob/50cfe0702793ab737dbe34f25994a05e7a22f921/contracts/src/Functions.sol#L94)

Decoding on rust side:

[Command.RegisterToken](https://github.com/paritytech/polkadot-sdk/blob/35d8868a9773cb560f00bd79f644228af6708d8f/bridges/snowbridge/primitives/inbound-queue/src/v1.rs#L37)

```rust
pub enum Command {
    /// Register a wrapped token on the AssetHub `ForeignAssets` pallet
    RegisterToken {
        /// The address of the ERC20 token to be bridged over to AssetHub
        token: H160,
        /// XCM execution fee on AssetHub
        fee: u128,
    },
}
```

Validations on rust side:

Message not supported yet, but will likely be implemented as part of this processor, or a similar in this file:

[NativeTokenTransferMessageProcessor](https://github.com/moondance-labs/tanssi/blob/3a1e91756e1b5f6cfd40590db9ee4e133fc4a8b0/chains/orchestrator-relays/runtime/starlight/src/bridge_to_ethereum_config.rs#L156C12-L156C47)

## SendToken

Anyone can call it.

TODO

# Starlight -> Ethereum

[Command](https://github.com/moondance-labs/tanssi/blob/3a1e91756e1b5f6cfd40590db9ee4e133fc4a8b0/primitives/bridge/src/lib.rs#L83)

Ethereum entry point: 

[Gateway.submitV1](https://github.com/moondance-labs/tanssi-bridge-relayer/blob/8f563fb5af462eadf9881a1b00153170a2c8cb1a/overridden_contracts/src/Gateway.sol#L180)

Callable by anyone.

Only reverts on
* ChannelDoesNotExist
* InvalidNonce (messages must be relayed in order)
* InvalidProof (BEEFY proof)
* NotEnoughGas

If message handler reverts, it is simply ignored. Sets `success = false`, but increases the nonce
and keeps processing following messages.

## ReportRewards
It is used to report rewards of operators producing blocks on starlight.

One message sent per starlight era, on era end.


### Payload

[Command.ReportRewards](https://github.com/moondance-labs/tanssi/blob/3a1e91756e1b5f6cfd40590db9ee4e133fc4a8b0/primitives/bridge/src/lib.rs#L86C1-L99C7)

```rust
pub enum Command {
    /// snippet
    ReportRewards {
        // external identifier for validators
        external_idx: u64,
        // index of the era we are sending info of
        era_index: u32,
        // total_points for the era
        total_points: u128,
        // new tokens inflated during the era
        tokens_inflated: u128,
        // merkle root of vec![(validatorId, rewardPoints)]
        rewards_merkle_root: H256,
        // the token id in which we need to mint
        token_id: H256,
    },
}
```

```rust
pub fn abi_encode(&self) -> Vec<u8> {
    let external_idx_token = Token::Uint(U256::from(*external_idx));
    let era_index_token = Token::Uint(U256::from(*era_index));
    let total_points_token = Token::Uint(U256::from(*total_points));
    let tokens_inflated_token = Token::Uint(U256::from(*tokens_inflated));
    let rewards_mr_token = Token::FixedBytes(rewards_merkle_root.0.to_vec());
    let token_id_token = Token::FixedBytes(token_id.0.to_vec());

    ethabi::encode(&[Token::Tuple(vec![
        external_idx_token,
        era_index_token,
        total_points_token,
        tokens_inflated_token,
        rewards_mr_token,
        token_id_token,
    ])])
}
```

Rust validations:

[amount == 0 || totalPoints == 0](https://github.com/moondance-labs/tanssi/blob/3a1e91756e1b5f6cfd40590db9ee4e133fc4a8b0/pallets/external-validators-rewards/src/lib.rs#L299)

Message not sent if no rewards. (can happen if only whitelisted validators produce blocks)

Snowbridge: only called by self, so only when it receives the message

[Gateway.sendRewards](https://github.com/moondance-labs/tanssi-bridge-relayer/blob/8f563fb5af462eadf9881a1b00153170a2c8cb1a/overridden_contracts/src/Gateway.sol#L526)

Symbiotic: only called be middleware:

[ODefaultOperatorRewards.distributeRewards](https://github.com/moondance-labs/tanssi-symbiotic/blob/e81b715b4c4c0a1e121ecafcf85a789faea642d2/src/contracts/rewarder/ODefaultOperatorRewards.sol#L128)

Ethereum validations:

[revert Middleware__InsufficientBalance](https://github.com/moondance-labs/tanssi-symbiotic/blob/e81b715b4c4c0a1e121ecafcf85a789faea642d2/src/contracts/middleware/Middleware.sol#L269)

[amount == 0 || totalPoints == 0](https://github.com/moondance-labs/tanssi-symbiotic/blob/e81b715b4c4c0a1e121ecafcf85a789faea642d2/src/contracts/rewarder/ODefaultOperatorRewards.sol#L137)

Redundant

Sends tokens from Middleware to ODefaultOperatorRewards



This payload is sent from starlight as outbound command. The timestamp here is originally the same timestamp sent from ethereum side as part of ReceiveValidators command. The `rewards_merkle_root` here refers to markle tree of reward data per validators.
These are the same validators sent by ethereum in ReceiveValidators command.

### Where and when the payload is created?
This payload is created in pallet external-validator-rewards in the `on_era_end` handler. This handler is triggered at the ending of an `era`.

### Where this payload is consumed?
This payload is consumed by our implementation of Symbiotic middleware when called by Gateway to handle rewards. It is consumed in function called `distributeRewards` in the middleware.

## ReportSlashes
It is used to report slashing of operators producing blocks on starlight.

Slashes are reported at the start of the next era, limited to 10 slashes per starlight block.

Snowbridge: only called by self, so only when it receives the message

[Gateway.reportSlashes](https://github.com/moondance-labs/tanssi-bridge-relayer/blob/8f563fb5af462eadf9881a1b00153170a2c8cb1a/overridden_contracts/src/Gateway.sol#L498)

Symbiotic: only called by middleware:

[Middleware.slash](https://github.com/moondance-labs/tanssi-symbiotic/blob/e81b715b4c4c0a1e121ecafcf85a789faea642d2/src/contracts/middleware/Middleware.sol#L345)


### Payload

[Command.ReportSlashes](https://github.com/moondance-labs/tanssi/blob/3a1e91756e1b5f6cfd40590db9ee4e133fc4a8b0/primitives/bridge/src/lib.rs#L100)

```rust
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

```rust
pub fn abi_encode(&self) -> Vec<u8> {
    let era_index_token = Token::Uint(U256::from(*era_index));
    let mut slashes_tokens_vec: Vec<Token> = vec![];

    for slash in slashes.into_iter() {
        let account_token = Token::FixedBytes(slash.encoded_validator_id.clone());
        let slash_fraction_token = Token::Uint(U256::from(slash.slash_fraction));
        let external_idx = Token::Uint(U256::from(slash.external_idx));
        let tuple_token =
            Token::Tuple(vec![account_token, slash_fraction_token, external_idx]);

        slashes_tokens_vec.push(tuple_token);
    }

    let slashes_tokens_array = Token::Array(slashes_tokens_vec);
    ethabi::encode(&[Token::Tuple(vec![era_index_token, slashes_tokens_array])])
}
```

This payload is sent from starlight as outbound command. The era_index here is starlight's era index in which the slashing has occurred. The ethereum side maintains a mapping from starlight era index to ethereum timestamp. This allows ethereum side to deduce for which epoch we need to slash a validator. (This will be changed in future.)

### Where and when the payload is created?
The payload is created in `external-validator-slashes` pallet in `process_slashes_queue` and handed off to outbound queue pallet. 

### Where this payload is consumed?
Each slash is handled by Middleware in a function called `slash`.

## Snowbridge outbound queue commands

Starlight -> Ethereum

[Command](https://github.com/paritytech/polkadot-sdk/blob/bade694abb5d43469395168f06f9c6f1dab26407/bridges/snowbridge/primitives/outbound-queue/src/v1/message.rs#L58)

These are maintenance and operations commands:

* AgentExecute
* Upgrade
* SetOperatingMode
* SetTokenTransferFees
* SetPricingParameters
* UnlockNativeToken
* RegisterForeignToken
* MintForeignToken

All commands are `ensure_root` origin, so users cannot trigger them manually.

Example: upgrade gateway command:

[EthereumSystem::upgrade](https://github.com/paritytech/polkadot-sdk/blob/bade694abb5d43469395168f06f9c6f1dab26407/bridges/snowbridge/pallets/system/src/lib.rs#L273)