export async function makeSendTokenMessageFrontierTemplate(isStarlight: boolean) {
    // Hard-coding payload as we do not have scale encoding-decoding
    // Payload with the following shape:
    // let payload = VersionedXcmMessage::V1(MessageV1 {
    //     chain_id: 1,
    //     command: Command::SendNativeToken {
    //         token_id: 0x485f805cb9de38b4324485447c664e16035aa9d28e8723df192fa02ad3530889,
    //         destination: Destination::ForeignAccountId20 {
    //             para_id: 2001,
    //             id: [5u; 20],
    //             fee: 500_000_000_000_000,
    //         },
    //         amount: 100_000_000,
    //         fee: 1_500_000_000_000_000,
    //     },
    // });
    let encodedMsgHex =
        "00010000000000000002485f805cb9de38b4324485447c664e16035aa9d28e8723df192fa02ad353088902d1070000050505050505050505050505050505050505050500406352bfc60100000000000000000000e1f50500000000000000000000000000c029f73d5405000000000000000000";
    if (isStarlight) {
        encodedMsgHex = encodedMsgHex.replace(
            "485f805cb9de38b4324485447c664e16035aa9d28e8723df192fa02ad3530889",
            "f0bc0f912407e1bc37bbd931f67bf0c7727b413d96a7d4663a6f0c1b00abb800"
        );
    }
    return Uint8Array.from(Buffer.from(encodedMsgHex, "hex"));
}

export async function makeSendTokenMessageFrontierTemplateInexistingParachain(isStarlight: boolean) {
    // Hard-coding payload as we do not have scale encoding-decoding
    // Payload with the following shape:
    // let payload = VersionedXcmMessage::V1(MessageV1 {
    //     chain_id: 1,
    //     command: Command::SendNativeToken {
    //         token_id: 0xbd2f49affab256f40ab8f040a591576f4b84ef70dc3ccddc367a19d829f6e604,
    //         destination: Destination::ForeignAccountId20 {
    //             para_id: 5000,
    //             id: [5u; 20],
    //             fee: 500_000_000_000_000,
    //         },
    //         amount: 100_000_000,
    //         fee: 1_500_000_000_000_000,
    //     },
    // });
    let encodedMsgHex =
        "00010000000000000002bd2f49affab256f40ab8f040a591576f4b84ef70dc3ccddc367a19d829f6e6040288130000050505050505050505050505050505050505050500406352bfc60100000000000000000000e1f50500000000000000000000000000c029f73d5405000000000000000000";
    if (isStarlight) {
        // Calculate token location using this command, change genesis hash to dancelight/starlight:
        /*
        ./target/release/tanssi-utils payload-generator   --token-location '{"parents": 0, "interior": {"X2": [{"Parachain": 5000}, {"PalletInstance": 10}]}}'   --para-id 2001   --beneficiary 0x0505050505050505050505050505050505050505   --container-fee 500000000000000   --amount 100000000   --fee 1500000000000000   --destination container   --token native --genesis-hash dd6d086f75ec041b66e20c4186d327b23c8af244c534a2418de6574e8c041a60
         */
        encodedMsgHex = encodedMsgHex.replace(
            "bd2f49affab256f40ab8f040a591576f4b84ef70dc3ccddc367a19d829f6e604",
            "0f7d55b1346b641257eafcd3979188e2bfc14fe7ab5691867ddb4e00d00496c3"
        );
    }
    return Uint8Array.from(Buffer.from(encodedMsgHex, "hex"));
}
