# Benchmarking
This guide explains how to use the benchmarking tool under `tools/benchmarking.sh` for better developer experience

## Benchmarking pallets vs benchmarking runtimes
Let's first explain the difference between benchmarking a pallet and benchmarking a runtime. When we benchmark a pallet, a public `trait WeightInfo` is created. The pallet is going to ask for an implementation of this trait in the associated `Config` type. Obviously when we benchmark a pallet this trait is by default implemented for the empty tuple and generic  `substrateWeight` struct. Here is an example:

```
/// Weight functions needed for pallet_data_preservers.
pub trait WeightInfo {
	fn set_boot_nodes(x: u32, y: u32, ) -> Weight;
}

/// Weights for pallet_data_preservers using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `Registrar::RegistrarDeposit` (r:1 w:0)
	/// Proof: `Registrar::RegistrarDeposit` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `DataPreservers::BootNodes` (r:0 w:1)
	/// Proof: `DataPreservers::BootNodes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `x` is `[1, 200]`.
	/// The range of component `y` is `[1, 10]`.
	fn set_boot_nodes(x: u32, y: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `195`
		//  Estimated: `3660`
		// Minimum execution time: 10_703_000 picoseconds.
		Weight::from_parts(9_788_229, 3660)
			// Standard Error: 170
			.saturating_add(Weight::from_parts(7_964, 0).saturating_mul(x.into()))
			// Standard Error: 3_552
			.saturating_add(Weight::from_parts(334_296, 0).saturating_mul(y.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: `Registrar::RegistrarDeposit` (r:1 w:0)
	/// Proof: `Registrar::RegistrarDeposit` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `DataPreservers::BootNodes` (r:0 w:1)
	/// Proof: `DataPreservers::BootNodes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `x` is `[1, 200]`.
	/// The range of component `y` is `[1, 10]`.
	fn set_boot_nodes(x: u32, y: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `195`
		//  Estimated: `3660`
		// Minimum execution time: 10_703_000 picoseconds.
		Weight::from_parts(9_788_229, 3660)
			// Standard Error: 170
			.saturating_add(Weight::from_parts(7_964, 0).saturating_mul(x.into()))
			// Standard Error: 3_552
			.saturating_add(Weight::from_parts(334_296, 0).saturating_mul(y.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
```

When we benchmark a runtime, we generate structs that implement the `WeightInfo` trait from all the pallets. This means that we don't create a new trait specific for a runtime:

```
/// Weights for pallet_data_preservers using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_data_preservers::WeightInfo for SubstrateWeight<T> {
	/// Storage: `Registrar::RegistrarDeposit` (r:1 w:0)
	/// Proof: `Registrar::RegistrarDeposit` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `DataPreservers::BootNodes` (r:0 w:1)
	/// Proof: `DataPreservers::BootNodes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `x` is `[1, 200]`.
	/// The range of component `y` is `[1, 10]`.
	fn set_boot_nodes(x: u32, y: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `195`
		//  Estimated: `3660`
		// Minimum execution time: 15_901_000 picoseconds.
		Weight::from_parts(13_983_853, 3660)
			// Standard Error: 154
			.saturating_add(Weight::from_parts(12_442, 0).saturating_mul(x.into()))
			// Standard Error: 3_215
			.saturating_add(Weight::from_parts(452_262, 0).saturating_mul(y.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}
```
The two files generated are quite different, hence the reason for us having two different templates to do benchmarking, which can be found at `benchmarking/frame-weight-runtime-template.hbs` and `benchmarking/frame-weight-template.hbs`.

## Using the benchmarking tool

The first thing we need to do is compile all runtime with `runtime-benchmarks` and `fast-runtime` features:

```
cargo build --features=fast-runtime,runtime-benchmarks --release
```

This will get the binaries ready for benchmarking purposes.

The next step is to to use the `tools/benchmarking.sh` script. There are four environmental variables you can set before using this tool:

- `BINARY`: The binary you want to use for benchmarking. If not specified, by default uses `target/release/tanssi-node`
- `CHAIN`: The chain that you want to use. By default it uses `dev`
- `OUTPUT_PATH`: The output path for the generated benchmarks. By default it uses `tmp`
- `TEMPLATE_PATH`: The template to use to generate the benchmarking file. By default it uses the pallet one, i.e., `benchmarking/frame-weight-pallet-template`.

Additional, the script is going to ask for two arguments:
- the pallet that you want to benchmark. If you want to benchmark all pallets, you need to pass `"*"`. Otherwise, you pase under quotes the pallet to be benchmarked, e.g., `"pallet_pooled_staking"`.
- the extrinsic that you want to benchmark. If you want to benchmark all extrinsics, you pass `"*"`. Otherwise you pass under quotes the extrinsic to be benchmarked, e.g., `"request_delegate"`

## Useful examples

### Benchmarking all pallets for the dancebox runtime

```
TEMPLATE_PATH=benchmarking/frame-weight-runtime-template.hbs OUTPUT_PATH=runtime/dancebox/src/weights ./tools/benchmarking.sh "*" "*"
```

### Benchmarking all pallets for the flashbox runtime

```
TEMPLATE_PATH=benchmarking/frame-weight-runtime-template.hbs OUTPUT_PATH=runtime/flashbox/src/weights CHAIN=flashbox_dev ./tools/benchmarking.sh "*" "*"
```

### Benchmarking all pallets for the container-chain-frontier-runtime

```
BINARY=target/release/container-chain-frontier-node TEMPLATE_PATH=benchmarking/frame-weight-runtime-template.hbs OUTPUT_PATH=container-chains/templates/frontier/runtime/src/weights ./tools/benchmarking.sh "*" "*"
```

### Benchmarking all pallets for the container-chain-simple-runtime

```
BINARY=target/release/container-chain-simple-node TEMPLATE_PATH=benchmarking/frame-weight-runtime-template.hbs OUTPUT_PATH=container-chains/templates/simple/runtime/src/weights ./tools/benchmarking.sh "*" "*"
```

### Generating weight info trait bound for pallet-pooled-staking

```
TEMPLATE_PATH=benchmarking/frame-weight-pallet-template.hbs OUTPUT_PATH=pallets/pooled-staking/src/weights.rs ./tools/benchmarking.sh "pallet_pooled_staking" "*"
```

### Generating weight info trait bound for pallet-cc-authorities-noting

```
BINARY=target/release/container-chain-simple-node TEMPLATE_PATH=benchmarking/frame-weight-pallet-template.hbs OUTPUT_PATH=../dancekit/container-chain-pallets/authorities-noting/src/weights.rs ./tools/benchmarking.sh "pallet_cc_authorities_noting" "*"
```
