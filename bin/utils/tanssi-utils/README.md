## Rewards utils generator tool

### Usage

Ensure you are located in the root of the Tanssi repo.

Then build the binary:

```bash
cargo build -p tanssi-utils --release
```

After that, run the tool with:

```bash
./target/release/tanssi-utils reward-claim-generator --input-path ./bin/utils/tanssi-utils/src/test-values.json
```

You can change the values present in `test-values.json` file to generate different utils as needed.

If the execution is successful, you should see an output like this one:

```bash
Input path is: "./bin/utils/tanssi-utils/src/test-values.json"

=== Era Rewards Utils: Overall info ===

Era index       : 1
Merkle Root     : 0x4b0ddd8b9b8ec6aec84bcd2003c973254c41d976f6f29a163054eec4e7947810
Total Points    : 40
Leaves:
  [0] 0x7aa72ad48826fe03d6a4bbf2598e1753fb7641efbbad5daacea9bd6a0bf7a507
  [1] 0x27e610a11a547f210646001377ae223bc6bce387931f8153624d21f6478512d2

=== Merkle Proofs ===

Merkle proof for account 0404040404040404040404040404040404040404040404040404040404040404 (5C9yEy27...) in era 1: 

   - Root: 0x4b0ddd8b9b8ec6aec84bcd2003c973254c41d976f6f29a163054eec4e7947810
   - Proof: [0x27e610a11a547f210646001377ae223bc6bce387931f8153624d21f6478512d2]
   - Number of leaves: 2
   - Leaf index: 0
   - Leaf: 0x7aa72ad48826fe03d6a4bbf2598e1753fb7641efbbad5daacea9bd6a0bf7a507

Merkle proof for account 0505050505050505050505050505050505050505050505050505050505050505 (5CBHb3Lf...) in era 1: 

   - Root: 0x4b0ddd8b9b8ec6aec84bcd2003c973254c41d976f6f29a163054eec4e7947810
   - Proof: [0x7aa72ad48826fe03d6a4bbf2598e1753fb7641efbbad5daacea9bd6a0bf7a507]
   - Number of leaves: 2
   - Leaf index: 1
   - Leaf: 0x27e610a11a547f210646001377ae223bc6bce387931f8153624d21f6478512d2
```