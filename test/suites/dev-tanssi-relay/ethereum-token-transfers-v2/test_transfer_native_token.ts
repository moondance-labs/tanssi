// @ts-nocheck

import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type ApiPromise, Keyring } from "@polkadot/api";
import {
    SEPOLIA_SOVEREIGN_ACCOUNT_ADDRESS,
    type MultiLocation,
    ETHEREUM_MAINNET_SOVEREIGN_ACCOUNT_ADDRESS,
    SNOWBRIDGE_FEES_ACCOUNT,
} from "utils";
import { expectEventCount } from "../../../helpers/events";
import { STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_SNOWBRIDGE_V2 } from "helpers";
import type { KeyringPair } from "@moonwall/util";

describeSuite({
    id: "DTR2001",
    title: "EthereumTokenTransfersV2 tests",
    foundationMethods: "dev",

    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let isStarlight: boolean;
        let specVersion: number;
        let shouldSkipStarlightETT: boolean;
        let sovereignAccount: string;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            const keyring = new Keyring({ type: "sr25519" });
            alice = keyring.addFromUri("//Alice", { name: "Alice default" });

            const runtimeName = polkadotJs.runtimeVersion.specName.toString();
            isStarlight = runtimeName === "starlight";
            specVersion = polkadotJs.consts.system.version.specVersion.toNumber();
            shouldSkipStarlightETT =
                isStarlight && STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_SNOWBRIDGE_V2.includes(specVersion);
            sovereignAccount = isStarlight
                ? ETHEREUM_MAINNET_SOVEREIGN_ACCOUNT_ADDRESS
                : SEPOLIA_SOVEREIGN_ACCOUNT_ADDRESS;
        });

        it({
            id: "E01",
            title: "transferNativeToken should send message to Ethereum",
            test: async () => {
                const tokenLocation: MultiLocation = {
                    parents: 0,
                    interior: "Here",
                };
                const versionedLocation = {
                    V3: tokenLocation,
                };

                const metadata = {
                    name: "dance",
                    symbol: "dance",
                    decimals: 12,
                };

                const registrarLocation = {
                    V3: {
                        parents: 0,
                        interior: "Here",
                    },
                };

                if (shouldSkipStarlightETT) {
                    console.log(`Skipping E01 test for Starlight version ${specVersion}`);
                    return;
                }

                const sudoSignedTx1 = await polkadotJs.tx.sudo.sudo(tx1).signAsync(alice);
                await context.createBlock([sudoSignedTx1], { allowFailures: false });

                // Register token on EthereumSystemV2.
                const tx2 = await polkadotJs.tx.sudo
                    .sudo(
                        polkadotJs.tx.ethereumSystemV2.registerToken(registrarLocation, versionedLocation, metadata, 0)
                    )
                    .signAsync(alice);

                await context.createBlock([tx2], { allowFailures: false });

                const recipient = "0x0000000000000000000000000000000000000007";
                const amount = 1000n;
                const relayerReward = 100n;

                const outboundNonceBefore = await polkadotJs.query.ethereumOutboundQueueV2.nonce();

                const snowbridgeFeesAccountBalanceBefore = (
                    await polkadotJs.query.system.account(SNOWBRIDGE_FEES_ACCOUNT)
                ).data.free.toBigInt();
                const sovereignAccountBalanceBefore = (
                    await polkadotJs.query.system.account(sovereignAccount)
                ).data.free.toBigInt();

                // Finally call transferNativeToken extrinsic.
                const tx3 = await polkadotJs.tx.ethereumTokenTransfers
                    .transferNativeTokenV2(amount, recipient, relayerReward)
                    .signAsync(alice);
                await context.createBlock([tx3], { allowFailures: false });

                // Things to verify:
                // 1. ethereumOutboundQueueV2 increases the nonce
                // 2. reward goes to snowbridge fees account
                // 3. sent tokens go to sovereign account
                // 4. a pending order exists for such nonce, with the fee=reward
                const outboundNonceAfter = await polkadotJs.query.ethereumOutboundQueueV2.nonce();

                const snowbridgeFeesAccountBalanceAfter = (
                    await polkadotJs.query.system.account(SNOWBRIDGE_FEES_ACCOUNT)
                ).data.free.toBigInt();

                const sovereignAccountBalanceAfter = (
                    await polkadotJs.query.system.account(sovereignAccount)
                ).data.free.toBigInt();

                const pendingOrder = await polkadotJs.query.ethereumOutboundQueueV2.pendingOrders(outboundNonceAfter);

                expect(outboundNonceAfter.toNumber()).to.be.equal(outboundNonceBefore.toNumber() + 1);
                expect(snowbridgeFeesAccountBalanceAfter).to.be.eq(snowbridgeFeesAccountBalanceBefore + relayerReward);
                expect(sovereignAccountBalanceAfter).to.be.eq(sovereignAccountBalanceBefore + amount);
                expect(pendingOrder.unwrap().fee.toBigInt()).to.be.equal(relayerReward);

                // Check events and digest were emitted correctly.
                // Should have resulted in a new "other" digest log being included in the block
                const baseHeader = await polkadotJs.rpc.chain.getHeader();
                const allLogs = baseHeader.digest.logs.map((x) => x.toJSON());
                const otherLogs = allLogs.filter((x) => x.other);
                expect(otherLogs.length).to.be.equal(1);
                const logHex = otherLogs[0].other;

                await expectEventCount(polkadotJs, {
                    MessagesCommitted: 1,
                    MessageAccepted: 1,
                    Processed: 1,
                    MessageQueued: 1,
                    NativeTokenTransferred: 1,
                });

                // Also a MessagesCommitted event with the same hash as the digest log
                const events = await polkadotJs.query.system.events();
                const ev1 = events.filter((a) => {
                    return a.event.method === "MessagesCommitted";
                });
                expect(ev1.length).to.be.equal(1);
                const ev1Data = ev1[0].event.data[0].toJSON();

                // logHex == 0x01 + ev1Data
                // Example:
                // logHex: 0x0164cf0ef843ad5a26c2cc27cf345fe0fd8b72cd6297879caa626c4d72bbe4f9b0
                // ev1Data:  0x64cf0ef843ad5a26c2cc27cf345fe0fd8b72cd6297879caa626c4d72bbe4f9b0
                const prefixedEv1Data = `0x01${ev1Data.slice(2)}`;
                expect(prefixedEv1Data).to.be.equal(logHex);
            },
        });
    },
});
