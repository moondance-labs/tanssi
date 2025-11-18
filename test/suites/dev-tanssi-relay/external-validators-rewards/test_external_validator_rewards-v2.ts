// @ts-nocheck

import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { KeyringPair } from "@moonwall/util";
import { type ApiPromise, Keyring } from "@polkadot/api";
import {
    SEPOLIA_SOVEREIGN_ACCOUNT_ADDRESS,
    ETHEREUM_MAINNET_SOVEREIGN_ACCOUNT_ADDRESS,
    type MultiLocation,
    jumpToSession,
    USE_V2_STORAGE_KEY,
} from "utils";
import { STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_SNOWBRIDGE_V2 } from "helpers";

describeSuite({
    id: "DEVT0604",
    title: "Ethereum reward tests v2",
    foundationMethods: "dev",

    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let runtimeName: string;
        let sovereignAccountToCheck: string;
        let isStarlight: boolean;
        let specVersion: number;
        let shouldSkipStarlightETT: boolean;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            alice = context.keyring.alice;
            runtimeName = polkadotJs.runtimeVersion.specName.toString();
            if (runtimeName === "starlight") {
                sovereignAccountToCheck = ETHEREUM_MAINNET_SOVEREIGN_ACCOUNT_ADDRESS;
            } else {
                sovereignAccountToCheck = SEPOLIA_SOVEREIGN_ACCOUNT_ADDRESS;
            }

            specVersion = polkadotJs.consts.system.version.specVersion.toNumber();
            isStarlight = runtimeName === "starlight";

            shouldSkipStarlightETT =
                isStarlight && STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_SNOWBRIDGE_V2.includes(specVersion);

            if (shouldSkipStarlightETT) {
                console.log(`Skipping E01 test for Starlight version ${specVersion}`);
                return;
            }
            // TRUE is defined as 0x01
            await context.createBlock(
                await polkadotJs.tx.sudo
                    .sudo(polkadotJs.tx.system.setStorage([[USE_V2_STORAGE_KEY, "0x01"]]))
                    .signAsync(alice)
            );
            const keyring = new Keyring({ type: "sr25519" });
            const aliceStash = keyring.addFromUri("//Alice//stash");
            // We need to register the token otherwise rewards are not sent to ethereum
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

            // Register Alice as an external validator, because it starts as a whitelisted validator and whitelisted
            // validators don't get rewards.
            let aliceNonce = (await polkadotJs.rpc.system.accountNextIndex(alice.address)).toNumber();

            await context.createBlock([
                await polkadotJs.tx.sudo
                    .sudo(polkadotJs.tx.externalValidators.removeWhitelisted(aliceStash.address))
                    .signAsync(context.keyring.alice, { nonce: aliceNonce++ }),
                await polkadotJs.tx.sudo
                    .sudo(polkadotJs.tx.externalValidators.setExternalValidators([aliceStash.address], 1))
                    .signAsync(context.keyring.alice, { nonce: aliceNonce++ }),
                await polkadotJs.tx.sudo
                    .sudo(polkadotJs.tx.ethereumSystem.registerToken(versionedLocation, metadata))
                    .signAsync(context.keyring.alice, { nonce: aliceNonce++ }),
            ]);
        });

        it({
            id: "E01",
            title: "Ethereum Sovereign Account balance should increase on era change",
            test: async () => {
                if (shouldSkipStarlightETT) {
                    console.log(`Skipping E01 test for Starlight version ${specVersion}`);
                    return;
                }
                const currentIndex = await polkadotJs.query.session.currentIndex();
                const sessionsPerEra = polkadotJs.consts.externalValidators.sessionsPerEra;

                const {
                    data: { free: balanceBefore },
                } = await context.polkadotJs().query.system.account(sovereignAccountToCheck);

                // And nonce too
                const mainChannelNonceBefore = await context.polkadotJs().query.ethereumOutboundQueueV2.nonce();

                // We need to jump at least 2 eras
                // One for the validator changes to take effect
                // another one to generate rewards
                const targetSession = currentIndex.toNumber() + sessionsPerEra.toNumber() * 2;

                await jumpToSession(context, targetSession);

                const {
                    data: { free: balanceAfter },
                } = await context.polkadotJs().query.system.account(sovereignAccountToCheck);
                const mainChannelNonceAfter = await context.polkadotJs().query.ethereumOutboundQueueV2.nonce();

                expect(balanceAfter.toBigInt()).to.be.greaterThan(balanceBefore.toBigInt());
                expect(mainChannelNonceAfter.toNumber()).to.be.equal(mainChannelNonceBefore.toNumber() + 1);
            },
        });
    },
});
