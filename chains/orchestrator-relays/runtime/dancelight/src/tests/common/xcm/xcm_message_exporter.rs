use {
    super::{
        super::{AccountId, ExtBuilder, ALICE},
        mocknets::{DancelightRelay as Dancelight, DancelightRelayPallet},
    },
    frame_support::weights::Weight,
    pallet_xcm::Error,
    sp_runtime::DispatchError,
    xcm::{latest::prelude::*, VersionedXcm},
    xcm_emulator::Chain,
};

#[test]
fn test_message_export_disabled() {
    ExtBuilder::default().build().execute_with(|| {
        // The only test we can do is with signed runtime origins since we are ensuring local origin in xcm config
        let origin = <Dancelight as Chain>::RuntimeOrigin::signed(AccountId::from(ALICE));

        let message = Xcm(vec![Instruction::ExportMessage {
            network: NetworkId::Ethereum { chain_id: 1 },
            destination: Junctions::Here.into(),
            xcm: Xcm(vec![]),
        }]);

        assert_eq!(
            <Dancelight as DancelightRelayPallet>::XcmPallet::execute(
                origin,
                Box::new(VersionedXcm::V4(message)),
                Weight::from_parts(0, 0)
            )
            .unwrap_err()
            .error,
            DispatchError::from(Error::<<Dancelight as Chain>::Runtime>::LocalExecutionIncomplete)
        );
    });
}
