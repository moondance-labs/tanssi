Compile the code with signer for offchain worker
```sh
cargo build --release --features ocw
```

Run the development node
```sh
./target/release/cyborg-node --dev --enable-offchain-indexing=true
```