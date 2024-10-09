# Ethereum State Verification Diagram

## Altair Sync protocol

### Preliminaries
- Ethereum epochs consis of 32 slots, where each slot is 12s
- Ethereum distributes all validators in attestation 32 attestation committees (one per slot)
- At the beginning of each epoch, each validator is assigned a slot
- Ethereum pseudo-randomly selects, among all validators, a block producer for a slot n
- The block produced for slot n needs to be attested by the attestation committee for slot n


### The need for a sync protocol
- For a light client, determining the canonical chain can become expensive (as we need to verify a bunch of attestions)
- In order to verify a block header, you need the information of the previous header

### The Altair sync protocol
- A sync committe made up of 512 pseudo-randomly chosen validators is selected every 256 epochs (approx, 27hours)
- Each block header includes the aggregated signature of the validators in the sync committee,
- Each header lists the current and the next sync committee
- Sync committee members are rewarded with 0.1ETH for their service, and they are charged that amount if they dont sign
- 2/3rds quorum needs to be reached
- For now sync committee members are not slashed if the vote for a malicious header (although subject to change after EIP-7657)

### Verification PseudoCode

```python
def validate_light_client_update(snapshot: LightClientSnapshot,
                                 update: LightClientUpdate,
                                 genesis_validators_root: Root) -> None:
    # Verify update slot is larger than snapshot slot
    assert update.header.slot > snapshot.header.slot

    # Verify update does not skip a sync committee period
    snapshot_period = compute_epoch_at_slot(snapshot.header.slot) // EPOCHS_PER_SYNC_COMMITTEE_PERIOD
    update_period = compute_epoch_at_slot(update.header.slot) // EPOCHS_PER_SYNC_COMMITTEE_PERIOD
    assert update_period in (snapshot_period, snapshot_period + 1)

    # Verify update header root is the finalized root of the finality header, if specified
    if update.finality_header == BeaconBlockHeader():
        signed_header = update.header
        assert update.finality_branch == [Bytes32() for _ in range(floorlog2(FINALIZED_ROOT_INDEX))]
    else:
        signed_header = update.finality_header
        assert is_valid_merkle_branch(
            leaf=hash_tree_root(update.header),
            branch=update.finality_branch,
            depth=floorlog2(FINALIZED_ROOT_INDEX),
            index=get_subtree_index(FINALIZED_ROOT_INDEX),
            root=update.finality_header.state_root,
        )

    # Verify update next sync committee if the update period incremented
    if update_period == snapshot_period:
        sync_committee = snapshot.current_sync_committee
        assert update.next_sync_committee_branch == [Bytes32() for _ in range(floorlog2(NEXT_SYNC_COMMITTEE_INDEX))]
    else:
        sync_committee = snapshot.next_sync_committee
        assert is_valid_merkle_branch(
            leaf=hash_tree_root(update.next_sync_committee),
            branch=update.next_sync_committee_branch,
            depth=floorlog2(NEXT_SYNC_COMMITTEE_INDEX),
            index=get_subtree_index(NEXT_SYNC_COMMITTEE_INDEX),
            root=update.header.state_root,
        )

    # Verify sync committee has sufficient participants
    assert sum(update.sync_committee_bits) >= MIN_SYNC_COMMITTEE_PARTICIPANTS

    # Verify sync committee aggregate signature
    participant_pubkeys = [pubkey for (bit, pubkey) in zip(update.sync_committee_bits, sync_committee.pubkeys) if bit]
    domain = compute_domain(DOMAIN_SYNC_COMMITTEE, update.fork_version, genesis_validators_root)
    signing_root = compute_signing_root(signed_header, domain)
    assert bls.FastAggregateVerify(participant_pubkeys, signing_root, update.sync_committee_signature)
```

## Transmiting Ethereum Headers to Starlight

### The relayers job to transmit Ethereum's header
1. The relayer node needs to collect the aggergated sync committee signature embeedded into the Ethereum block
2. The relayer needs to aggregrate the public keys responsible for generating that block.
3. If the sync committee changes, the relayer needs to generate a merkle proof proving that the sync committee has changed.
4. The relayer presents all these components to the appropriate pallet in starlight

### Eth state receiver pallets job
1. If the relayer claims the sync committee changes, verify the merkle proof to make sure the new sync committee is correct.
2. Perform a verification of the validity of the committee members that signed the aggregated message
3. Perform a verification of the aggregated signature
4. If all of those pass, accept the header, with the new **state root ready to receive storage proofs**

### Overall Diagram ETH state validity Reception

<p align="center">
  <img src="images/beacon_chain_altair.drawio.png" width="1000">
</p>



## Proving symbiotic validator selection to starlight (or any other Ethereum state)

### The relayers job to transmit symbiotic validators
1. The relayer node checks the latest accepted header in starlight, and it generates a storage proof of the validators storage item in the symbiotic smart contract
2. The relayer sends this information to the Symbiotic validator receiver pallet

### The Symbiotic validator Receiver pallets job to verify the validators

1. The symbiotic validator pallet receiver asks the Ethereum state receiver pallet about the correctness of the validators.
2. The ethereum state pallet receiver repplies with a yes or no, depending on the validity of the storage proof against the storage root stored
3. If accepted, the next validator set is stored on-chain
4. In the next session, the new era will get started

### Overall Diagram Validator Information passing

<p align="center">
  <img src="images/relayer_proof_starlight.drawio.png" width="1000">
</p>
