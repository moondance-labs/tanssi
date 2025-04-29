import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { KeyringPair } from "@moonwall/util";
import type { ApiPromise } from "@polkadot/api";
import { initializeCustomCreateBlock } from "utils";
import { STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_PROXY, checkCallIsFiltered } from "helpers";

describeSuite({
    id: "DEVT1502",
    title: "Proxy test suite",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let sudoAlice: KeyringPair;
        let delegateBob: KeyringPair;
        const VALIDATOR_PROXY_INDEX = 8;
        let isStarlight: boolean;
        let specVersion: number;
        let shouldSkipStarlightProxy: boolean;

        beforeAll(() => {
            initializeCustomCreateBlock(context);

            sudoAlice = context.keyring.alice;
            delegateBob = context.keyring.bob;

            polkadotJs = context.polkadotJs();

            const runtimeName = polkadotJs.runtimeVersion.specName.toString();
            isStarlight = runtimeName === "starlight";
            specVersion = polkadotJs.consts.system.version.specVersion.toNumber();
            shouldSkipStarlightProxy = isStarlight && STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_PROXY.includes(specVersion);
        });

        it({
            id: "E01",
            title: "Can add proxy",
            test: async () => {
                await context.createBlock();

                const tx = polkadotJs.tx.proxy.addProxy(delegateBob.address, VALIDATOR_PROXY_INDEX, 0);

                if (shouldSkipStarlightProxy) {
                    console.log(`Skipping E01 test for Starlight version ${specVersion}`);
                    await checkCallIsFiltered(context, polkadotJs, await tx.signAsync(sudoAlice));
                    return;
                }

                await context.createBlock([await tx.signAsync(sudoAlice)]);

                const proxies = await polkadotJs.query.proxy.proxies(sudoAlice.address);
                expect(proxies.toJSON()[0]).to.deep.equal([
                    {
                        delegate: delegateBob.address,
                        proxyType: "SudoValidatorManagement",
                        delay: 0,
                    },
                ]);
            },
        });

        it({
            id: "E02",
            title: "Delegated account can sudo txs in external validators",
            test: async () => {
                const txAddWhitelisted = polkadotJs.tx.proxy.proxy(
                    sudoAlice.address,
                    null,
                    polkadotJs.tx.sudo.sudo(polkadotJs.tx.externalValidators.addWhitelisted(delegateBob.address))
                );

                if (shouldSkipStarlightProxy) {
                    console.log(`Skipping E02 test for Starlight version ${specVersion}`);
                    await checkCallIsFiltered(context, polkadotJs, await txAddWhitelisted.signAsync(delegateBob));
                    return;
                }

                await context.createBlock([await txAddWhitelisted.signAsync(delegateBob)]);

                const whitelistedValidatorInfo = await polkadotJs.query.externalValidators.whitelistedValidators();
                expect(whitelistedValidatorInfo.toHuman().includes(delegateBob.address)).to.be.true;
            },
        });

        it({
            id: "E03",
            title: "Delegated account can sudo txs in external validator slashes",
            test: async () => {
                const txAddWhitelisted = polkadotJs.tx.proxy.proxy(
                    sudoAlice.address,
                    null,
                    polkadotJs.tx.sudo.sudo(
                        polkadotJs.tx.externalValidatorSlashes.forceInjectSlash(0, sudoAlice.address, 1000, 1)
                    )
                );

                if (shouldSkipStarlightProxy) {
                    console.log(`Skipping E03 test for Starlight version ${specVersion}`);
                    await checkCallIsFiltered(context, polkadotJs, await txAddWhitelisted.signAsync(delegateBob));
                    return;
                }

                await context.createBlock([await txAddWhitelisted.signAsync(delegateBob)]);

                const DeferPeriod = (await polkadotJs.consts.externalValidatorSlashes.slashDeferDuration).toNumber();

                // scheduled slashes
                const expectedSlashes = await polkadotJs.query.externalValidatorSlashes.slashes(DeferPeriod + 1);
                expect(expectedSlashes.length).to.be.eq(1);
            },
        });
    },
});
