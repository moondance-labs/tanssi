use cumulus_pallet_parachain_system::RelayChainStateProof;
use cumulus_primitives_core::relay_chain::{BlakeTwo256, BlockNumber};
use hex_literal::hex;
use parity_scale_codec::Decode;
use parity_scale_codec::Encode;
use sp_consensus_aura::{inherents::InherentType, AURA_ENGINE_ID};
use sp_runtime::DigestItem;
use test_relay_sproof_builder::{HeaderAs, ParaHeaderSproofBuilder, ParaHeaderSproofBuilderItem};
use tp_core::well_known_keys::para_id_head;

#[test]
fn header_decode_collisions() {
    // The hex below is the result of encoding a Header to Vec<u8>, and then encoding that Vec<u8> again.
    // Trying to decode this bytes directly as a Header should always fail, but because of how the
    // SCALE codec works it can sometimes succeed and output garbage.
    let bad_value = hex!("e102ad81ae5c9623edf94e9ca481698383ac8032e13a8a0642407a51987e98a5d5db01010fcbe894fb15e253e2918af5633a040bd379fa5d225685101fa5e8d17843c68de9e6d71f42d894088c1cfb6d4ee9d2bf9abc5254428dcadc4997442007afb6e00806617572612048a659080000000005617572610101dc4e2be503910fb326840244eb65fe21d9a9a8f23414ab909f3baabb991e8855abd5a00f1640ec8df48687f33967887f4a86ae6299693e9baf28b7192722248d");
    let good_value = hex!("e102451c84b3d0383f1d7002fd597c45406bd8d2c0bace9e52bb35a8dbfa805b46c60501888d8570e847209a707668977b5792569e865796a9130e1c37fdb1fd7c6f3b73e87cbecebd0de4abd17b8a80995972d8187ae9998a87d134b807e9b8f5565e2b0806617572612049a6590800000000056175726101010ee968af2eac0ce1223b5618497961064542543a75b72abad1a7d919fc7d8937a4180c242670561c4179e8b83cedde3e80cfc99793b5a35cf020055fc80cb684");
    let bad: Result<sp_runtime::generic::Header<BlockNumber, BlakeTwo256>, _> =
        <_>::decode(&mut &bad_value[..]);
    let good: Result<sp_runtime::generic::Header<BlockNumber, BlakeTwo256>, _> =
        <_>::decode(&mut &good_value[..]);

    assert!(bad.is_err());
    assert!(good.is_ok());

    // But decoding as a Vec<u8> and then as a Header will always work.
    let bad: Result<sp_runtime::generic::Header<BlockNumber, BlakeTwo256>, _> =
        <Vec<u8>>::decode(&mut &bad_value[..]).and_then(|bytes| <_>::decode(&mut &bytes[..]));
    let good: Result<sp_runtime::generic::Header<BlockNumber, BlakeTwo256>, _> =
        <Vec<u8>>::decode(&mut &good_value[..]).and_then(|bytes| <_>::decode(&mut &bytes[..]));

    assert!(bad.is_ok());
    assert!(good.is_ok());
}

fn test_header() -> sp_runtime::generic::Header<u32, BlakeTwo256> {
    let slot: InherentType = 13u64.into();

    sp_runtime::generic::Header::<u32, BlakeTwo256> {
        parent_hash: Default::default(),
        number: Default::default(),
        state_root: Default::default(),
        extrinsics_root: Default::default(),
        digest: sp_runtime::generic::Digest {
            logs: vec![DigestItem::PreRuntime(AURA_ENGINE_ID, slot.encode())],
        },
    }
}

#[test]
fn header_double_encode() {
    // The ParaHeaderSproofBuilder should always encode as a Vec<u8>, and then encode that Vec<u8> again.
    let mut s = ParaHeaderSproofBuilderItem::default();
    s.para_id = 1001.into();
    let header = test_header();
    let header_encoded = header.encode();
    s.author_id = HeaderAs::NonEncoded(header);

    let mut sb = ParaHeaderSproofBuilder::default();
    sb.items.push(s);
    let (state_root, proof) = sb.into_state_root_and_proof();

    let relay_state_proof = RelayChainStateProof::new(1001.into(), state_root, proof)
        .expect("Invalid relay chain state proof");
    let key = para_id_head(1001.into());
    // If the NonEncoded was not encoded once to Vec, and then again as a Vec, this would fail
    // because we are comparing the "decoded" entry with the encoded header
    let v: Vec<u8> = relay_state_proof.read_entry(&key, None).unwrap();
    assert_eq!(v, header_encoded);
}

#[test]
fn header_double_encode_even_if_already_encoded() {
    // The ParaHeaderSproofBuilder should always encode as a Vec<u8>, and then encode that Vec<u8> again.
    let mut s = ParaHeaderSproofBuilderItem::default();
    s.para_id = 1001.into();
    let header = test_header();
    let header_encoded = header.encode();
    s.author_id = HeaderAs::AlreadyEncoded(header_encoded.clone());

    let mut sb = ParaHeaderSproofBuilder::default();
    sb.items.push(s);
    let (state_root, proof) = sb.into_state_root_and_proof();

    let relay_state_proof = RelayChainStateProof::new(1001.into(), state_root, proof)
        .expect("Invalid relay chain state proof");
    let key = para_id_head(1001.into());
    // If the AlreadyEncoded was not encoded again as a Vec, this would fail
    let v: Vec<u8> = relay_state_proof.read_entry(&key, None).unwrap();
    assert_eq!(v, header_encoded);
}
