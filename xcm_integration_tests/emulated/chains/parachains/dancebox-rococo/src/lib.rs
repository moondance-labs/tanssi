pub use sp_core::Get;
pub use xcm_emulator::TestExt;
use {
    emulated_integration_tests_common::xcm_emulator::decl_test_parachains,
    parity_scale_codec::Encode, sp_consensus_aura::AURA_ENGINE_ID, sp_runtime::generic::DigestItem,
};

mod genesis;

decl_test_parachains! {
pub struct DanceboxRococo {
        genesis = genesis::genesis(),
        on_init = {
            dancebox_runtime::System::deposit_log(DigestItem::PreRuntime(AURA_ENGINE_ID, 0u64.encode()));
        },
        runtime = dancebox_runtime,
        core = {
            XcmpMessageHandler: dancebox_runtime::XcmpQueue,
            LocationToAccountId: dancebox_runtime::xcm_config::LocationToAccountId,
            ParachainInfo: dancebox_runtime::ParachainInfo,
            MessageOrigin: cumulus_primitives_core::AggregateMessageOrigin,
        },
        pallets = {
            System: dancebox_runtime::System,
            Balances: dancebox_runtime::Balances,
            ParachainSystem: dancebox_runtime::ParachainSystem,
            PolkadotXcm: dancebox_runtime::PolkadotXcm,
            ForeignAssets:  dancebox_runtime::ForeignAssets,
            AssetRate:  dancebox_runtime::AssetRate,
            ForeignAssetsCreator: dancebox_runtime::ForeignAssetsCreator,
        }
    },
}
