// @ts-nocheck

import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type ApiPromise, Keyring } from "@polkadot/api";
import {
    generateUpdate,
    ETHEREUM_NETWORK_TESTNET,
    ETHEREUM_NETWORK_MAINNET,
    SEPOLIA_SOVEREIGN_ACCOUNT_ADDRESS,
    ETHEREUM_MAINNET_SOVEREIGN_ACCOUNT_ADDRESS,
    generateOutboundMessageAcceptedLog,
} from "utils";
import type { KeyringPair } from "@moonwall/util";
import { STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_SNOWBRIDGE_V2 } from "../../../helpers";

describeSuite({
    id: "ETHINBV2SYSTEMREMARK",
    title: "Snowbridge InboundQueueV2 receive raw xcm transact",
    foundationMethods: "dev",

    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let isStarlight: boolean;
        let ethNetworkId: number;
        let shouldSkipStarlightSnV2TT: boolean;
        let specVersion: number;
        let sovereignAccountAddress: string;
        let balanceDiff1: bigint | undefined;

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
            title: "Execute xcm transact 1 system remark",
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
                            originKind: "SovereignAccount",
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

                const balanceBefore = (await polkadotJs.query.system.account(alice.address)).data.free.toBigInt();
                const tx3 = await polkadotJs.tx.ethereumInboundQueueV2.submit(messageExtrinsics[0]).signAsync(alice);
                await context.createBlock([tx3], { allowFailures: false });

                const events = await polkadotJs.query.system.events();
                const remarkedEvents = events.filter((event) => event.toHuman().event.method === "Remarked");
                expect(remarkedEvents.length).to.equal(1);

                const balanceAfter = (await polkadotJs.query.system.account(alice.address)).data.free.toBigInt();

                // Reward is 0, so alice balance must be lower after submitting this inbound message
                balanceDiff1 = balanceBefore - balanceAfter;
                expect(balanceAfter < balanceBefore).to.be.true;

                // Reward is stored in bridgeRelayers pallet and must be manually claimed. Assert that the reward is zero.
                const pendingReward = await polkadotJs.query.bridgeRelayers.relayerRewards(
                    alice.address,
                    "SnowbridgeRewardInbound"
                );
                expect(pendingReward.isNone).to.equal(true);
            },
        });

        it({
            id: "E02",
            title: "Execute xcm transact 100 system remark is more expensive than 1 system remark",
            test: async () => {
                if (shouldSkipStarlightSnV2TT) {
                    console.log("Skipping test for Starlight runtime");
                    return;
                }
                const transferAmount = 0n;
                const batchTxs = [];
                for (let i = 0; i < 100; i++) {
                    batchTxs.push(polkadotJs.tx.system.remarkWithEvent(`remark number ${i}`));
                }
                const systemRemarkCall = polkadotJs.tx.utility.batchAll(batchTxs);
                const systemRemarkCallEncoded = systemRemarkCall?.method.toHex();

                const instructions = [
                    {
                        Transact: {
                            originKind: "SovereignAccount",
                            requireWeightAtMost: null,
                            call: {
                                encoded: systemRemarkCallEncoded,
                            },
                        },
                    },
                ];

                const log = await generateOutboundMessageAcceptedLog(polkadotJs, 2, transferAmount, instructions);

                const { checkpointUpdate, messageExtrinsics } = await generateUpdate(polkadotJs, [log]);

                const tx = polkadotJs.tx.ethereumBeaconClient.forceCheckpoint(checkpointUpdate);
                const signedTx = await polkadotJs.tx.sudo.sudo(tx).signAsync(alice);
                await context.createBlock([signedTx], { allowFailures: false });

                const balanceBefore = (await polkadotJs.query.system.account(alice.address)).data.free.toBigInt();
                const tx3 = await polkadotJs.tx.ethereumInboundQueueV2.submit(messageExtrinsics[0]).signAsync(alice);
                await context.createBlock([tx3], { allowFailures: false });

                const events = await polkadotJs.query.system.events();
                const remarkedEvents = events.filter((event) => event.toHuman().event.method === "Remarked");
                expect(remarkedEvents.length).to.equal(100);
                const balanceAfter = (await polkadotJs.query.system.account(alice.address)).data.free.toBigInt();

                // Reward is 0, so alice balance must be lower after submitting this inbound message
                const balanceDiff2 = balanceBefore - balanceAfter;
                expect(balanceAfter < balanceBefore).to.be.true;

                // Assert that executing 100 system remark is more expensive than executing 1 system remark
                // It is not 100 times more expensive because the submit extrinsic does other things aside from the remark
                expect(balanceDiff2 > balanceDiff1).to.be.true;

                // Reward is stored in bridgeRelayers pallet and must be manually claimed. Assert that the reward is zero.
                const pendingReward = await polkadotJs.query.bridgeRelayers.relayerRewards(
                    alice.address,
                    "SnowbridgeRewardInbound"
                );
                expect(pendingReward.isNone).to.equal(true);
            },
        });

        it({
            id: "E03",
            title: "Try to execute overweight xcm message fails with overweight error",
            test: async () => {
                if (shouldSkipStarlightSnV2TT) {
                    console.log("Skipping test for Starlight runtime");
                    return;
                }
                const transferAmount = 0n;
                // InboundQueueV2 xcm execution is limited to 25% of the block weight.
                // So try to execute an xcm message that takes 50% of the block weight, it should fail.
                const BILLION = 1_000_000_000n;
                const perBill = (50n * BILLION) / 100n;
                // We can use any extrinsic that takes more than 25% of the block weight.
                // Use rootTesting.fillBlock because it allows to explicitly specify the weight.
                // This extrinsic can only be called by root, so we cannot use it to test that a smaller weight works.
                // But we can use it to create a weight error because the weight is calculated before the origin check.
                const systemRemarkCall = polkadotJs.tx.rootTesting.fillBlock(perBill);
                const systemRemarkCallEncoded = systemRemarkCall?.method.toHex();

                const instructions = [
                    {
                        Transact: {
                            originKind: "SovereignAccount",
                            requireWeightAtMost: null,
                            call: {
                                encoded: systemRemarkCallEncoded,
                            },
                        },
                    },
                ];

                const log = await generateOutboundMessageAcceptedLog(polkadotJs, 3, transferAmount, instructions);

                const { checkpointUpdate, messageExtrinsics } = await generateUpdate(polkadotJs, [log]);

                const tx = polkadotJs.tx.ethereumBeaconClient.forceCheckpoint(checkpointUpdate);
                const signedTx = await polkadotJs.tx.sudo.sudo(tx).signAsync(alice);
                await context.createBlock([signedTx], { allowFailures: false });

                const balanceBefore = (await polkadotJs.query.system.account(alice.address)).data.free.toBigInt();
                const tx3 = await polkadotJs.tx.ethereumInboundQueueV2.submit(messageExtrinsics[0]).signAsync(alice);
                await context.createBlock([tx3], { allowFailures: false });

                const events = await polkadotJs.query.system.events();
                const xcmEvents = events.filter((event) => event.toHuman().event.method === "Attempted");
                expect(xcmEvents.length).to.equal(1);
                const xcmEvent = xcmEvents[0];
                const weightLimitReachedError = xcmEvent.event.data[0].toJSON().error.error.weightLimitReached;
                expect(
                    weightLimitReachedError,
                    `expected weightLimitReached error, found unexpected xcm error: ${JSON.stringify(xcmEvent.event.data.toJSON())}`
                ).to.not.be.undefined;

                const balanceAfter = (await polkadotJs.query.system.account(alice.address)).data.free.toBigInt();

                // Reward is 0, so alice balance must be lower after submitting this inbound message
                const balanceDiff3 = balanceBefore - balanceAfter;
                expect(balanceAfter < balanceBefore).to.be.true;

                // Assert that an execution error is less expensive than executing 1 system remark
                expect(balanceDiff3 < balanceDiff1).to.be.true;

                // Reward is stored in bridgeRelayers pallet and must be manually claimed. Assert that the reward is zero.
                const pendingReward = await polkadotJs.query.bridgeRelayers.relayerRewards(
                    alice.address,
                    "SnowbridgeRewardInbound"
                );
                expect(pendingReward.isNone).to.equal(true);
            },
        });
    },
});
