use cumulus_primitives_core::relay_chain::HeadData;
use cumulus_primitives_core::ParaId;
use frame_support::Hashable;
use sp_runtime::traits::BlakeTwo256;
use sp_runtime::traits::HashFor;
use sp_trie::MemoryDB;

#[derive(Clone)]
pub enum HeaderAs {
    AlreadyEncoded(Vec<u8>),
    NonEncoded(sp_runtime::generic::Header<u32, BlakeTwo256>),
}

/// Builds a sproof (portmanteau of 'spoof' and 'proof') of the relay chain state.
#[derive(Clone)]
pub struct AuthorNotingSproofBuilder {
    /// The para id of the current parachain.
    ///
    /// This doesn't get into the storage proof produced by the builder, however, it is used for
    /// generation of the storage image and by auxiliary methods.
    ///
    /// It's recommended to change this value once in the very beginning of usage.
    ///
    /// The default value is 200.
    pub para_id: ParaId,

    pub author_id: HeaderAs,
}

impl AuthorNotingSproofBuilder {
    pub fn default() -> Self {
        AuthorNotingSproofBuilder {
            para_id: ParaId::from(200),
            author_id: HeaderAs::AlreadyEncoded(vec![]),
        }
    }

    pub fn into_state_root_and_proof(
        self,
    ) -> (
        polkadot_primitives::v2::Hash,
        sp_state_machine::StorageProof,
    ) {
        let (db, root) = MemoryDB::<HashFor<polkadot_primitives::v2::Block>>::default_with_root();
        let state_version = Default::default(); // for test using default.
        let mut backend = sp_state_machine::TrieBackendBuilder::new(db, root).build();

        let mut relevant_keys = Vec::new();
        {
            use parity_scale_codec::Encode as _;

            let mut insert = |key: Vec<u8>, value: Vec<u8>| {
                relevant_keys.push(key.clone());
                backend.insert(vec![(None, vec![(key, Some(value))])], state_version);
            };

            let para_key = self.para_id.twox_64_concat();
            let key = [crate::PARAS_HEADS_INDEX, para_key.as_slice()].concat();

            log::info!("key in sproof is {:?}", key);

            let encoded = match self.author_id {
                HeaderAs::AlreadyEncoded(encoded) => encoded,
                HeaderAs::NonEncoded(header) => header.encode(),
            };

            let head_data: HeadData = encoded.into();
            insert(key, head_data.encode());
        }

        let root = backend.root().clone();
        let proof = sp_state_machine::prove_read(backend, relevant_keys).expect("prove read");

        (root, proof)
    }
}
