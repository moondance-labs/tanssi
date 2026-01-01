# LayerZero Integration Architecture

This document describes how Tanssi integrates with LayerZero to enable cross-chain messaging between container chains and external LayerZero-connected chains (Solana, Arbitrum, Base, etc.).

## Overview

The LayerZero integration enables bidirectional messaging:

- **Inbound**: Messages from LayerZero chains → Container chains
- **Outbound**: Messages from container chains → LayerZero chains

A **central hub smart contract on Ethereum** acts as the bridge between LayerZero and Tanssi:
- Receives messages from all LayerZero-connected chains
- Forwards them to Snowbridge for delivery to container chains
- Receives outbound messages from container chains and sends them via LayerZero

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         LAYERZERO-CONNECTED CHAINS                           │
│                                                                              │
│  ┌──────────────┐    ┌──────────────┐    ┌──────────────┐                   │
│  │    Solana    │    │   Arbitrum   │    │     Base     │   ...             │
│  │  (LZ: 30168) │    │  (LZ: 30110) │    │  (LZ: 30184) │                   │
│  └──────┬───────┘    └──────┬───────┘    └──────┬───────┘                   │
│         │                   │                   │                            │
│         └───────────────────┴───────────────────┘                            │
│                             │                                                │
│                    ┌────────▼────────┐                                       │
│                    │    LayerZero    │                                       │
│                    │    Protocol     │                                       │
│                    └────────┬────────┘                                       │
└─────────────────────────────┼───────────────────────────────────────────────┘
                              │
┌─────────────────────────────┼───────────────────────────────────────────────┐
│                          ETHEREUM                                            │
│                              │                                               │
│              ┌───────────────▼───────────────┐                               │
│              │   TanssiLzHub                 │◄── Central hub: receives all  │
│              │   (LayerZero Receiver)        │    LZ messages from other     │
│              │                               │    chains, forwards to        │
│              │   • Receives from Solana      │    Snowbridge                 │
│              │   • Receives from Arbitrum    │                               │
│              │   • Receives from Base, etc.  │                               │
│              └───────────────┬───────────────┘                               │
│                              │                                               │
│              ┌───────────────▼───────────────┐                               │
│              │   Snowbridge Gateway          │◄── Bridges messages to        │
│              │   Contract                    │    Tanssi relay chain         │
│              └───────────────┬───────────────┘                               │
└──────────────────────────────┼──────────────────────────────────────────────┘
                               │
                      Ethereum Light Client
                               │
┌──────────────────────────────┼──────────────────────────────────────────────┐
│                     TANSSI RELAY CHAIN                                       │
│                              │                                               │
│              ┌───────────────▼───────────────┐                               │
│              │  EthereumInboundQueueV2       │◄── Receives messages from     │
│              │  (Snowbridge)                 │    Ethereum via light client  │
│              └───────────────┬───────────────┘                               │
│                              │                                               │
│              ┌───────────────▼───────────────┐                               │
│              │  LayerZeroMessageProcessor    │◄── Extracts & validates       │
│              │  (runtime-common/processors)  │    LayerZero messages         │
│              └───────────────┬───────────────┘                               │
│                              │                                               │
│              ┌───────────────▼───────────────┐                               │
│              │      pallet-lz-router         │◄── Routes messages to/from    │
│              │                               │    container chains via XCM   │
│              │  • RoutingConfigs             │                               │
│              │  • handle_inbound_message()   │                               │
│              │  • send_message_to_ethereum() │                               │
│              └───────────────┬───────────────┘                               │
│                              │                                               │
│                         XCM (DMP)                                            │
│                              │                                               │
└──────────────────────────────┼──────────────────────────────────────────────┘
                               │
        ┌──────────────────────┼──────────────────────┐
        │                      │                      │
        ▼                      ▼                      ▼
┌───────────────┐      ┌───────────────┐      ┌───────────────┐
│ Container     │      │ Container     │      │ Container     │
│ Chain 2000    │      │ Chain 2001    │      │ Chain N       │
│               │      │               │      │               │
│ ┌───────────┐ │      │ ┌───────────┐ │      │ ┌───────────┐ │
│ │ Receiver  │ │      │ │ Receiver  │ │      │ │ Receiver  │ │
│ │ Pallet    │ │      │ │ Pallet    │ │      │ │ Pallet    │ │
│ └───────────┘ │      │ └───────────┘ │      │ └───────────┘ │
└───────────────┘      └───────────────┘      └───────────────┘
```

## Inbound Message Flow (External Chain → Container Chain)

### Step-by-Step Flow

```
1. User/Contract  ────►  2. TanssiLzSpoke ────►  3. LayerZero  ────►  4. TanssiLzHub
   triggers send            on source            Protocol            on Ethereum
   on source chain          chain                delivers             receives msg
                                                                           │
       ┌───────────────────────────────────────────────────────────────────┘
       │
       ▼
5. Snowbridge     ────►  6. Relay Chain  ────►  7. LzRouter   ────►  8. Container Chain
   Gateway               InboundQueue           validates &          receives via XCM
   forwards              receives               forwards             Transact
```

### Detailed Steps

1. **User/Contract**: A user or smart contract on Solana/Arbitrum/Base triggers a message send targeting a Tanssi container chain
2. **TanssiLzSpoke**: The Tanssi-owned spoke contract on the source chain receives the request and sends it via LayerZero to the Ethereum hub
3. **LayerZero Protocol**: Delivers the message to Ethereum
4. **TanssiLzHub**: The central hub contract on Ethereum receives the LayerZero message and forwards it to the Snowbridge Gateway
5. **Snowbridge Gateway**: Emits an event that the Ethereum light client picks up
6. **EthereumInboundQueueV2**: Receives the message on the relay chain via the light client
7. **LayerZeroMessageProcessor**: Extracts and validates the LayerZero payload
8. **LzRouter**: `handle_inbound_message()` is called:
   - Looks up the routing config for the destination chain
   - Validates the sender (endpoint + address) is whitelisted
   - Constructs an XCM message with `Transact`
9. **XCM Delivery**: The message is sent via DMP (Downward Message Passing) to the container chain
10. **Container Chain**: The receiver pallet processes the message

### Message Structure

```rust
pub struct InboundMessage {
    pub lz_source_address: BoundedVec<u8, ConstU32<32>>,  // Source contract address (up to 32 bytes)
    pub lz_source_endpoint: u32,                           // LayerZero endpoint ID (e.g., 30101 for Ethereum)
    pub destination_chain: u32,                            // Target container chain ID
    pub message: Vec<u8>,                                  // Application payload
}
```

### XCM Message (Relay → Container Chain)

When `LzRouter` forwards a message to a container chain, it sends the following XCM:

```rust
Xcm([
    // Allow unpaid execution from the relay chain
    UnpaidExecution {
        weight_limit: Unlimited,
        check_origin: None,
    },
    // Call the configured pallet on the container chain
    Transact {
        origin_kind: OriginKind::Xcm,
        fallback_max_weight: None,
        call: encoded_call,  // (pallet_index, call_index, SCALE-encoded InboundMessage)
    },
])
```

The `encoded_call` is structured as:

```
┌─────────────────┬─────────────────┬──────────────────────────────────┐
│  pallet_index   │   call_index    │   SCALE-encoded InboundMessage   │
│     (u8)        │      (u8)       │         (Vec<u8>)                │
└─────────────────┴─────────────────┴──────────────────────────────────┘
```

**Example**: If the container chain configured `notification_destination: (79, 0)`:
- `pallet_index = 79` (the receiver pallet)
- `call_index = 0` (the `receive_message` extrinsic)
- The `InboundMessage` struct is SCALE-encoded and passed as the payload

The container chain's receiver pallet will decode and process the message.

## Outbound Message Flow (Container Chain → External Chain)

### Step-by-Step Flow

```
1. Container       2. LzRouter        3. Snowbridge      4. TanssiLzHub
   sends XCM  ────►  queues msg  ────►  relays to   ────►  sends via LZ
   to relay          for outbound      Ethereum           to dest chain
                                                                │
                                                                ▼
                                       6. Target Contract  5. TanssiLzSpoke
                                          receives msg  ◄────  relays to target
                                          from Tanssi         app contract
```

### Detailed Steps

1. **Container Chain**: Sends an XCM message to the relay chain calling `lz_router::send_message_to_ethereum()`
2. **LzRouter**: Validates the origin and queues the message for the Ethereum outbound queue
3. **Snowbridge Outbound**: Relays the message to Ethereum
4. **TanssiLzHub**: Receives the message and sends it via LayerZero to the destination chain
5. **LayerZero Protocol**: Delivers the message to the target chain (Solana, Arbitrum, etc.)
6. **TanssiLzSpoke**: The Tanssi-owned spoke contract on the destination chain receives the LayerZero message and relays it to the target application contract

## Ethereum Hub Contract

The **TanssiLzHub** contract on Ethereum is the central bridge between LayerZero and Tanssi:

```
                    ┌─────────────────────────────────────┐
                    │         TanssiLzHub                 │
                    │           (Ethereum)                │
                    │                                     │
   LayerZero ──────►│  lzReceive()                        │──────► Snowbridge
   Messages         │    • Validates source chain         │        Gateway
                    │    • Formats message for Snowbridge │        (to Tanssi)
                    │    • Calls Gateway.sendMessage()    │
                    │                                     │
   Snowbridge ─────►│  receiveFromTanssi()                │──────► LayerZero
   Messages         │    • Receives from Snowbridge       │        (to dest chain)
                    │    • Sends via LayerZero            │
                    └─────────────────────────────────────┘
```

### Responsibilities

- **Inbound**: Receives LayerZero messages from all connected chains and forwards them to Snowbridge Gateway
- **Outbound**: Receives messages from Snowbridge and sends them via LayerZero to destination chains
- **Single Entry Point**: All cross-chain messages go through this hub, simplifying the trust model

## Spoke Contracts (Other Chains)

The **TanssiLzSpoke** contracts are deployed on each LayerZero-connected chain (Solana, Arbitrum, Base, etc.) and act as entry/exit points for messages to/from Tanssi:

```
                    ┌─────────────────────────────────────┐
                    │       TanssiLzSpoke                 │
                    │   (Solana, Arbitrum, Base, etc.)    │
                    │                                     │
   User/App ───────►│  sendToTanssi()                     │──────► LayerZero
   on this chain    │    • Accepts destination chain ID   │        (to TanssiLzHub)
                    │    • Accepts payload bytes          │
                    │    • Sends via LayerZero to Hub     │
                    │                                     │
   LayerZero ──────►│  lzReceive()                        │──────► Target App
   (from Hub)       │    • Receives from TanssiLzHub      │        Contract
                    │    • Validates origin is Hub        │
                    │    • Forwards to destination app    │
                    └─────────────────────────────────────┘
```

### Responsibilities

- **Inbound (to Tanssi)**: Accepts user messages and sends them to `TanssiLzHub` on Ethereum via LayerZero
- **Outbound (from Tanssi)**: Receives messages from `TanssiLzHub` and delivers them to target application contracts
- **Origin Validation**: Only accepts incoming LayerZero messages from the trusted `TanssiLzHub` on Ethereum

## Configuration

### Container Chain Setup

Each container chain must configure its routing rules by sending an XCM message to the relay chain that calls `lz_router::update_routing_config()`:

```rust
RoutingConfig {
    // Which (endpoint, address) pairs are allowed to send messages to this chain
    whitelisted_senders: bounded_vec![
        (30101, bounded_vec![0x12, 0x34, ...]),  // Endpoint ID, contract address (up to 32 bytes)
    ],
    // Which pallet/extrinsic receives the messages
    notification_destination: (79, 0),  // (pallet_index, call_index)
}
```

### Receiver Pallet Requirements

A container chain receiver pallet must:

1. **Accept XCM origin from parent (relay)**:
   ```rust
   type ParentOrigin = pallet_xcm::EnsureXcm<Equals<ParentLocation>>;
   ```

2. **Have a receive extrinsic with matching signature**:
   ```rust
   pub fn receive_message(origin: OriginFor<T>, payload: Vec<u8>) -> DispatchResult {
       let origin_location = T::ParentOrigin::ensure_origin(origin)?;
       
       // The payload contains the SCALE-encoded InboundMessage
       // Decode it to access the LayerZero message fields:
       let inbound_message = InboundMessage::decode(&mut payload.as_slice())?;
       
       // Now you can access:
       // - inbound_message.lz_source_address  (sender contract)
       // - inbound_message.lz_source_endpoint (source chain endpoint ID)
       // - inbound_message.destination_chain  (this chain's ID)
       // - inbound_message.message            (application payload)
       
       Ok(())
   }
   ```

3. **Configure XCM barriers**:
   - `AllowExplicitUnpaidExecutionFrom<Equals<ParentLocation>>` in barrier
   - `pallet_xcm::XcmPassthrough<RuntimeOrigin>` in origin conversion

> **Note**: There is no requirement for a specific pallet name, call name, or call index. The pallet can be named anything and the extrinsic can have any name. What matters is:
> - The extrinsic signature matches: accepts XCM origin + `Vec<u8>` payload
> - The `notification_destination` in the routing config has the correct `(pallet_index, call_index)` that matches your runtime
>
> The `payload` parameter contains the SCALE-encoded `InboundMessage` struct. Container chains must decode it to access the original LayerZero message fields and implement their own custom logic.

See `pallet-lz-receiver-example` for a complete reference implementation.

## Key Components

| Component | Location | Purpose |
|-----------|----------|---------|
| **External Chains** | | |
| TanssiLzSpoke (Solana) | Solana | Entry point for Solana → Tanssi messages |
| TanssiLzSpoke (Arbitrum) | Arbitrum | Entry point for Arbitrum → Tanssi messages |
| TanssiLzSpoke (Base) | Base | Entry point for Base → Tanssi messages |
| TanssiLzSpoke (...) | Other LZ chains | Entry points on other LayerZero chains |
| **Ethereum** | | |
| TanssiLzHub | Ethereum | Central hub for all LZ messages |
| Snowbridge Gateway | Ethereum | Bridges to/from Tanssi relay chain |
| **Relay Chain** | | |
| `pallet-lz-router` | `pallets/lz-router/` | Routes messages between chains |
| `LayerZeroMessageProcessor` | `chains/runtime-common/src/processors/v2/` | Extracts LZ messages from Snowbridge |
| `EthereumInboundQueueV2` | Snowbridge | Receives messages from Ethereum |
| **Container Chain** | | |
| `pallet-lz-receiver-example` | `pallets/lz-receiver-example/` | Example receiver pallet |
| **Shared** | | |
| `tp_bridge::layerzero_message::InboundMessage` | `primitives/bridge/src/inbound_queue/` | Message types |

## Security Model

- **Whitelisting**: Each container chain explicitly whitelists which LayerZero senders can send messages to it
- **Origin Verification**: Messages are verified to come from the relay chain via XCM origin checks
- **Self-Configuration**: Only the container chain itself (via XCM) can update its routing configuration
- **Sender Validation**: Every inbound message is checked against the whitelist before forwarding
