import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type KeyringPair, alith } from "@moonwall/util";
import { type ApiPromise, Keyring } from "@polkadot/api";
import { encodeAddress } from "@polkadot/util-crypto";

import { SEPOLIA_SOVEREIGN_ACCOUNT_ADDRESS, generateEventLog, generateUpdate, signAndSendAndInclude, ETHEREUM_NETWORK_TESTNET } from "utils";

describeSuite({
    id: "ZOMBIETANSS02",
    title: "Container native tokens transfer from Ethereum to container (via Tanssi)",
    foundationMethods: "zombie",
    testCases: ({ context, it }) => {
        let containerChainPolkadotJs: ApiPromise;
        let relayChainPolkadotJs: ApiPromise;
        let aliceFrontier: KeyringPair;
        let aliceRelay: KeyringPair;
        let chain: string;

        // Random ETH destination that we send asset to
        const destinationAddress = "0x1234567890abcdef1234567890abcdef12345678";
        const holdingAccount = SEPOLIA_SOVEREIGN_ACCOUNT_ADDRESS;
        const tokenToTransfer = 123_321_000_000_000_000n;

        const newChannelId = "0x0000000000000000000000000000000000000000000000000000000000000001";
        const newAgentId = "0x0000000000000000000000000000000000000000000000000000000000000001";
        const newParaId = 0;

        beforeAll(async () => {
            containerChainPolkadotJs = context.polkadotJs("Container2001");
            relayChainPolkadotJs = context.polkadotJs("Tanssi-relay");
            chain = containerChainPolkadotJs.consts.system.version.specName.toString();
            aliceRelay = new Keyring({ type: "sr25519" }).addFromUri("//Alice", {
                name: "Alice default",
            });
            aliceFrontier = chain === "frontier-template" ? alith : aliceRelay;
        });

        it({
            id: "T01",
            title: "Should receive container native tokens from Ethereum and forward them to container",
            test: async () => {

                const ethereumSovereignAccount = await containerChainPolkadotJs.call.locationToAccountApi.convertLocation({
                    V3: { parents: 2, interior: { X1: { GlobalConsensus: ETHEREUM_NETWORK_TESTNET } } },
                });

                console.log("ethereumSovereignAccount: ", ethereumSovereignAccount.toHuman());
                const ethereumSovereignAccountAddress = ethereumSovereignAccount.asOk.toHuman();

                const transferAmount = BigInt(10_000);

                // Create token receiver account
                const tokenReceiver = encodeAddress(
                    "0x0505050505050505050505050505050505050505050505050505050505050505"
                );

                // Hard-coding payload as we do not have scale encoding-decoding
                const log = await generateEventLog(
                    relayChainPolkadotJs,
                    Uint8Array.from(Buffer.from("eda338e4dc46038493b885327842fd3e301cab39", "hex")),
                    Uint8Array.from(
                        Buffer.from("0000000000000000000000000000000000000000000000000000000000000004", "hex")
                    ),
                    Uint8Array.from(
                        Buffer.from("0000000000000000000000000000000000000000000000000000000000000000", "hex")
                    ),
                    1,
                    new Uint8Array([0, 1, 0, 0, 0, 0, 0, 0, 0, 2, 72, 95, 128, 92, 185, 222, 56, 180, 50, 68, 133, 68, 124, 102, 78, 22, 3, 90, 169, 210, 142, 135, 35, 223, 25, 47, 160, 42, 211, 83, 8, 137, 1, 209, 7, 0, 0, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 244, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 232, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
                );
                const { checkpointUpdate, messageExtrinsics } = await generateUpdate(relayChainPolkadotJs, [log]);

                const tx = relayChainPolkadotJs.tx.ethereumBeaconClient.forceCheckpoint(checkpointUpdate);

                await signAndSendAndInclude(relayChainPolkadotJs.tx.sudo.sudo(tx), aliceRelay);

                // Create EthereumTokenTransfers channel to validate when receiving the tokens.
                const newChannelId = "0x0000000000000000000000000000000000000000000000000000000000000004";
                const newAgentId = "0x0000000000000000000000000000000000000000000000000000000000000005";
                const newParaId = 500;

                const tx1 = await relayChainPolkadotJs.tx.sudo
                    .sudo(
                        relayChainPolkadotJs.tx.ethereumTokenTransfers.setTokenTransferChannel(
                            newChannelId,
                            newAgentId,
                            newParaId
                        )
                    )
                await signAndSendAndInclude(tx1, aliceRelay);

                const containerMetadata = await containerChainPolkadotJs.rpc.state.getMetadata();
                const balancesPalletIndex = containerMetadata.asLatest.pallets
                    .find(({ name }) => name.toString() === "Balances")
                    .index.toNumber();


                    const tokenLocation = {
                        parents: 0,
                        interior: {
                            X2: [
                                {
                                    Parachain: 2001,
                                },
                                {
                                    PalletInstance: balancesPalletIndex,
                                },
                            ],
                        },
                    };
                    const versionedLocation = {
                        V3: tokenLocation,
                    };
    
                    const metadata = {
                        name: "para2001",
                        symbol: "para2001",
                        decimals: 12,
                    };
    
                    // Register token on EthereumSystem.
                    const tx2 = relayChainPolkadotJs.tx.sudo
                        .sudo(relayChainPolkadotJs.tx.ethereumSystem.registerToken(versionedLocation, metadata))

                    await signAndSendAndInclude(tx2, aliceRelay);

                    const tx3 = relayChainPolkadotJs.tx.sudo.sudo(relayChainPolkadotJs.tx.paras.forceSetCurrentHead(2001, "0x010203"));

                    await signAndSendAndInclude(tx3, aliceRelay);

                    const tx4 = relayChainPolkadotJs.tx.sudo.sudo(relayChainPolkadotJs.tx.xcmPallet.forceDefaultXcmVersion(5));
                    await signAndSendAndInclude(tx4, aliceRelay);

                    console.log("LOG 1");

                    const relayTokenLocation = {
                        parents: 1,
                        interior: "Here"
                    }

                    const registerTanssiAssetTx = containerChainPolkadotJs.tx.sudo
                    .sudo(
                        containerChainPolkadotJs.tx.foreignAssetsCreator.createForeignAsset(
                            relayTokenLocation,
                            42,
                            aliceFrontier.address,
                            true,
                            1
                        )
                    );

                    console.log("LOG 2");

                    await signAndSendAndInclude(registerTanssiAssetTx, aliceFrontier);

                    const assetRateTx = containerChainPolkadotJs.tx.assetRate.create("42", 2_000_000_000_000_000_000n);
                    await signAndSendAndInclude(assetRateTx, aliceFrontier);

                    console.log("LOG 3");

                    const transferRelayToken = containerChainPolkadotJs.tx.foreignAssets.mint(42, ethereumSovereignAccountAddress, 2000000000000000n);

                    console.log("LOG 4");

                    await signAndSendAndInclude(transferRelayToken, aliceFrontier);

                    console.log("LOG 5");

                    const transferRelayTokenToAlice = containerChainPolkadotJs.tx.balances.transferKeepAlive(ethereumSovereignAccountAddress, 20000000000000000n);
                    await signAndSendAndInclude(transferRelayTokenToAlice, aliceFrontier);

                    console.log("LOG 6");

                    const tx5 = relayChainPolkadotJs.tx.ethereumInboundQueue.submit(messageExtrinsics[0]);
                    await signAndSendAndInclude(tx5, aliceRelay);



            },
        });
    },
});