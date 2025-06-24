# Introduction
This document outlines how the messaging works between starlight and ethereum. It will describe each type of message supported by either side and then 
will describe the general flow of the messages from creation to handling of it.

# Table of contents

<!--
Generate table of contents using
md_toc --in-place github --header-levels 2 docs/bridge/messages.md
-->

<!--TOC-->

- [Introduction](#introduction)
- [Table of contents](#table-of-contents)
- [Bridge architecture recap](#bridge-architecture-recap)
- [Ethereum -> Starlight (starlight processor)](#ethereum---starlight-starlight-processor)
  - [ReceiveValidators](#receivevalidators)
- [Ethereum -> Starlight (snowbridge processor)](#ethereum---starlight-snowbridge-processor)
  - [RegisterToken](#registertoken)
  - [SendToken](#sendtoken)
- [Starlight -> Ethereum](#starlight---ethereum)
  - [ReportRewards](#reportrewards)
  - [ReportSlashes](#reportslashes)
- [Snowbridge outbound queue commands](#snowbridge-outbound-queue-commands)

<!--TOC-->

# Bridge architecture recap

List repos here

* [tanssi](https://github.com/moondance-labs/tanssi)
  * Custom messages (starlight processor)
  * Pallets for slashing/rewards/external validators
* [polkadot-sdk](https://github.com/moondance-labs/polkadot-sdk/tree/tanssi-polkadot-stable-2412)
  * Snowbridge v1 pallets
  * Some custom cherry-picks to make snowbridge more generic
* [tanssi-bridge-relayer](https://github.com/moondance-labs/tanssi-bridge-relayer)
  * Solidity part of the custom starlight messages (overriden_contracts)
  * Relayers (go). Little changes from upstream, they never decode the messages. Only relay them.
* [tanssi-symbiotic](https://github.com/moondance-labs/tanssi-symbiotic)
  * (?) integration with symbiotic for tanssi
* [snowbridge](https://github.com/moondance-labs/snowbridge/tree/tanssi-relay-v1.0.40)
  * (?) another relayer? whats the difference with tanssi-bridge-relayer

TODO: maybe explain how to find the actual commit that is deployed for each repo

TODO: brief description of how a message gets relayed

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

### Where and when the payload is created?
This payload is created on ethereum side to send operators data to starlight. The timestamp here is an opaque type created by ethereum and not semantically interpreted by starlight.

### Where this payload is consumed?
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

### Who can send it and when

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

Send tokens from ethereum to starlight. Supports ERC20, ETH, and wrapped versions of starlight-native and container-native tokens

### Who can send it and when

[sendToken](https://github.com/Snowfork/snowbridge/blob/010806b1a94799dbb67c99569f0fd48be4c0a0a3/contracts/src/Gateway.sol#L311)

Anyone can call it. Token must have been registered previously, but anyone can register a token.

### Payload

```solidity
// Transfer ERC20 tokens to a Polkadot parachain
function sendToken(
    address token, // if 0x0, transfer ETH
    ParaID destinationChain,
    MultiAddress calldata destinationAddress,
    uint128 destinationFee,
    uint128 amount
) external payable nonreentrant {
```

[SendTokenToAssetHubAddress32](https://github.com/Snowfork/snowbridge/blob/50cfe0702793ab737dbe34f25994a05e7a22f921/contracts/src/SubstrateTypes.sol#L57)

SendTokenToAssetHubAddress32, SendTokenToAddress32, SendTokenToAddress20

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

Used to report rewards of operators producing blocks on starlight.

Rewards are stored in a merkle tree and must be manually claimed by operators.

### Who can send it and when

[ExternalValidatorsRewards::on_era_end](https://github.com/moondance-labs/tanssi/blob/f779e3ea2d822dc3efde12433612ab1a238d5f26/pallets/external-validators-rewards/src/lib.rs#L275)

This payload is created in pallet external-validator-rewards in the `on_era_end` handler. This handler is triggered at the ending of an `era`.

Sent automatically by runtime.

[1 era = 4 sessions](https://github.com/moondance-labs/tanssi/blob/f779e3ea2d822dc3efde12433612ab1a238d5f26/chains/orchestrator-relays/runtime/starlight/src/lib.rs#L1440)

[1 session = 6 hours](https://github.com/moondance-labs/tanssi/blob/f779e3ea2d822dc3efde12433612ab1a238d5f26/chains/orchestrator-relays/runtime/starlight/constants/src/lib.rs#L56)

So 1 message sent every 24 hours.

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

### Where this payload is consumed?
This payload is consumed by our implementation of Symbiotic middleware when called by Gateway to handle rewards. It is consumed in function called `distributeRewards` in the middleware.

## ReportSlashes

It is used to report slashing of operators producing blocks on starlight.

### Who can send it and when

[ExternalValidatorSlashes::process_slashes_queue](https://github.com/moondance-labs/tanssi/blob/f779e3ea2d822dc3efde12433612ab1a238d5f26/pallets/external-validator-slashes/src/lib.rs#L600)

The slash queue is populated in [ExternalValidatorSlashes::on_era_start](https://github.com/moondance-labs/tanssi/blob/f779e3ea2d822dc3efde12433612ab1a238d5f26/pallets/external-validator-slashes/src/lib.rs#L550)

Sent automatically by runtime at the start of the next era, similar to rewards (1 message every 24 hours).

Limited to [10 slashes](https://github.com/moondance-labs/tanssi/blob/f779e3ea2d822dc3efde12433612ab1a238d5f26/chains/orchestrator-relays/runtime/starlight/src/lib.rs#L1542) per starlight block.
If more than 10 slashes, will send up to 10 more on the next starlight block (6 seconds).

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

# Snowbridge outbound queue commands

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

[Gateway::upgrade](https://github.com/moondance-labs/tanssi-bridge-relayer/blob/8f563fb5af462eadf9881a1b00153170a2c8cb1a/overridden_contracts/src/Gateway.sol#L432)

