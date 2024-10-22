use crate::tests::common::ExtBuilder;
use crate::{BeefyMmrLeaf, PalletInfo, Runtime};
use beefy_primitives::mmr::BeefyAuthoritySet;
use frame_support::storage::unhashed;
use frame_support::traits::PalletInfo as _;
use pallet_migrations::Migration;
use tanssi_runtime_common::migrations::MigrateMMRLeafPallet;
use xcm::v3::Weight;

#[test]
fn test_migration_config_full_rotation_period() {
    ExtBuilder::default().build().execute_with(|| {
        let migrate_mmr_leaf_pallet = MigrateMMRLeafPallet::<Runtime>(Default::default());
        let old_pallet_name = MigrateMMRLeafPallet::<Runtime>::old_pallet_name();
        let old_storage_1 = frame_support::storage::storage_prefix(
            old_pallet_name.as_bytes(),
            b"example_storage_1",
        );
        let new_storage_1 = frame_support::storage::storage_prefix(
            PalletInfo::name::<BeefyMmrLeaf>()
                .expect("BeefyMMRLeaf pallet must be part of the runtime")
                .as_bytes(),
            b"example_storage_1",
        );
        unhashed::put(&old_storage_1, &1u64);

        let beefy_authority_set: BeefyAuthoritySet<()> = BeefyAuthoritySet {
            len: 5,
            ..Default::default()
        };
        let old_storage_2 = frame_support::storage::storage_prefix(
            old_pallet_name.as_bytes(),
            b"example_storage_2",
        );
        let new_storage_2 = frame_support::storage::storage_prefix(
            PalletInfo::name::<BeefyMmrLeaf>()
                .expect("BeefyMMRLeaf pallet must be part of the runtime")
                .as_bytes(),
            b"example_storage_2",
        );
        unhashed::put(&old_storage_2, &beefy_authority_set);

        let used_weight = migrate_mmr_leaf_pallet.migrate(Weight::MAX);
        assert_eq!(used_weight, Weight::MAX);

        assert_eq!(unhashed::get::<u64>(&old_storage_1), None);
        assert_eq!(unhashed::get::<BeefyAuthoritySet<()>>(&old_storage_2), None);

        assert_eq!(unhashed::get::<u64>(&new_storage_1), Some(1u64));
        assert_eq!(
            unhashed::get::<BeefyAuthoritySet<()>>(&new_storage_2),
            Some(BeefyAuthoritySet {
                len: 5,
                ..Default::default()
            })
        );
    });
}
