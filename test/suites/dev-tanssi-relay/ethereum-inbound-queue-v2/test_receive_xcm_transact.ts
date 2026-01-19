// @ts-nocheck

import "@tanssi/api-augment";

import { beforeAll, describeSuite } from "@moonwall/cli";
import { type ApiPromise, Keyring } from "@polkadot/api";
import {
    generateUpdate,
    ETHEREUM_NETWORK_TESTNET,
    ETHEREUM_NETWORK_MAINNET,
    encodeRawPayload,
    PayloadEnum,
    SEPOLIA_SOVEREIGN_ACCOUNT_ADDRESS,
    ETHEREUM_MAINNET_SOVEREIGN_ACCOUNT_ADDRESS,
    generateOutboundMessageAcceptedLog,
} from "utils";
import type { KeyringPair } from "@moonwall/util";
import { hexToU8a } from "@polkadot/util";
import { getBytes } from "ethers/utils";
import { AbiCoder } from "ethers/abi";
import { STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_SNOWBRIDGE_V2 } from "../../../helpers";

describeSuite({
    // TODO: this test suite should receive an xcm transact (system.remarkWithEvent) and assert that the event is emitted
    // and the relayer pays for the execution. So the message should have 0 value and 0 assets, and 0 reward. So the relayer
    // final balance should be less than the relayer initial balance.
    id: "ETHINBV2SYSTEMREMARK",
    title: "Receive Symbiotic update from Ethereum is failing",
    foundationMethods: "dev",

    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let isStarlight: boolean;
        let ethNetworkId: number;
        let shouldSkipStarlightSnV2TT: boolean;
        let specVersion: number;
        let sovereignAccountAddress: string;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            const keyring = new Keyring({ type: "sr25519" });
            alice = keyring.addFromUri("//Alice", { name: "Alice default" });

            const runtimeName = polkadotJs.runtimeVersion.specName.toString();
            isStarlight = runtimeName === "starlight";

            sovereignAccountAddress = isStarlight
                ? ETHEREUM_MAINNET_SOVEREIGN_ACCOUNT_ADDRESS
                : SEPOLIA_SOVEREIGN_ACCOUNT_ADDRESS;

            specVersion = polkadotJs.consts.system.version.specVersion.toNumber();
            shouldSkipStarlightSnV2TT =
                isStarlight && STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_SNOWBRIDGE_V2.includes(specVersion);

            if (shouldSkipStarlightSnV2TT) {
                console.log("Skipping test for Starlight runtime");
                return;
            }

            ethNetworkId = isStarlight ? ETHEREUM_NETWORK_MAINNET : ETHEREUM_NETWORK_TESTNET;
        });

        it({
            id: "E01",
            title: "Receive ETH native token from Ethereum in Tanssi chain",
            test: async () => {
                if (shouldSkipStarlightSnV2TT) {
                    console.log("Skipping test for Starlight runtime");
                    return;
                }
                const transferAmount = 0n;

                const systemRemarkCall = polkadotJs.tx.system.remarkWithEvent("0xaabbccdd");
                const systemRemarkCallEncoded = systemRemarkCall?.method.toHex();

                const instructions = [
                    {
                        Transact: {
                            originType: "SovereignAccount",
                            requireWeightAtMost: null,
                            call: {
                                encoded: systemRemarkCallEncoded,
                            },
                        },
                    },
                ];

                const log = await generateOutboundMessageAcceptedLog(polkadotJs, 1, transferAmount, instructions);

                const { checkpointUpdate, messageExtrinsics } = await generateUpdate(polkadotJs, [log]);

                const tx = polkadotJs.tx.ethereumBeaconClient.forceCheckpoint(checkpointUpdate);
                const signedTx = await polkadotJs.tx.sudo.sudo(tx).signAsync(alice);
                await context.createBlock([signedTx], { allowFailures: false });

                const tx3 = await polkadotJs.tx.ethereumInboundQueueV2.submit(messageExtrinsics[0]).signAsync(alice);
                await context.createBlock([tx3], { allowFailures: false });

                const events = await polkadotJs.query.system.events();
                console.log(events.toJSON());
                const remarkEventFound = events.find((event) => event.toHuman().event.method === "Remarked");
                expect(!!remarkEventFound).to.equal(true);
            },
        });
    },
});
