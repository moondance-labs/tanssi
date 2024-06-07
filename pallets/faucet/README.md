# Cyborg Usage

This is a modified and updated version of the pallet-faucet by Steve Degosserie: https://github.com/stiiifff/pallet-faucet

# Substrate Faucet pallet

The Faucet pallet allows an account to claim a pre-configured number of tokens up to a certain number of times, and not faster than a configured interval.

When an account claims tokens from the faucet, the pallet will mint new tokens and credit them to requestor account's balance, and the token's total issuance will be updated accordingly. The operation of claiming tokens is _feeless_ so the requesting account does not need to have any fund to submit the transaction.

This is a simple & effective way to distribute tokens on dev / test PoA chains, but **it should be not be used in any case on a public network** (which require more elaborate token distribution schemes baked by a solid cryptoeconomics model).

## Usage

### Claim tokens from the faucet

To claim tokens from the faucet, an account must send a transaction with a `faucet.claimTokens` extrinsic. The call is _feeless_ so no fee will be charged for the transaction, and the requestor account's balance will be credited with the number of tokens as configured in the `FaucetDripAmount` parameter of the pallet's `Config` trait in your runtime.

### Configure your runtime

#### Runtime `Cargo.toml`

To add this pallet to your runtime, simply include the following to your runtime's `Cargo.toml` file:

```TOML
[dependencies.pallet-faucet]
default_features = false
version = '1.0.0'
```

and update your runtime's `std` feature to include this pallet:

```TOML
std = [
    # --snip--
    'pallet-faucet/std',
]
```

#### Runtime `lib.rs`

You should implement it's Config trait like so:

```rust
parameter_types! {
    // Example configuration values.
    // Note: UNITS & DAYS constants might not be defined in your runtime.
    // 
    // The amount of token that "drips" from the faucet for every claim.
	pub const FaucetDripAmount: Balance = 1000 * UNITS;
    // The minimum period, as a number of blocks, between consecutive claims of a given account.
	pub const MinBlocksBetweenClaims: BlockNumber = 1 * DAYS;
    // The maximum number of times an account can claim tokens from the faucet.
	pub const MaxClaimsPerAccount: u32 = 3;
}

impl pallet_faucet::Config for Runtime {
	type Currency = Balances;
	type DripAmount = FaucetDripAmount;
	type Event = Event;
	type MaxClaimsPerAccount = MaxClaimsPerAccount;
	type MinBlocksBetweenClaims = MinBlocksBetweenClaims;
}
```

and include it in your `construct_runtime!` macro:

```rust
Faucet: pallet_faucet::{Pallet, Call, Storage, Event<T>},
```

### Genesis Configuration

This pallet does not have any genesis configuration.

## Testing

Run the tests with:

    ```
    cargo test
    ```
## Reference Docs

You can view the reference docs for this pallet by running:

```
cargo doc --open
```
## License

The Faucet pallet is licensed under [Apache 2](LICENSE).
