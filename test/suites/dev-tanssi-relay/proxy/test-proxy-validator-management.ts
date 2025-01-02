import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { KeyringPair } from "@moonwall/util";
import { ApiPromise } from "@polkadot/api";
import { initializeCustomCreateBlock } from "../../../util/block";

describeSuite({
    id: "DTR1201",
    title: "Proxy test suite",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let sudoAlice: KeyringPair;
        let delegateBob: KeyringPair;
        const VALIDATOR_PROXY_INDEX = 8;

        beforeAll(() => {
            initializeCustomCreateBlock(context);

            sudoAlice = context.keyring.alice;
            delegateBob = context.keyring.bob;

            polkadotJs = context.polkadotJs();
        });

        it({
            id: "E01",
            title: "Can add proxy",
            test: async function () {
                await context.createBlock();

                const tx = polkadotJs.tx.proxy.addProxy(delegateBob.address, VALIDATOR_PROXY_INDEX, 0);
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
            test: async function () {
                const txAddWhitelisted = polkadotJs.tx.proxy.proxy(
                    sudoAlice.address,
                    null,
                    polkadotJs.tx.sudo.sudo(polkadotJs.tx.externalValidators.addWhitelisted(delegateBob.address))
                );
                await context.createBlock([await txAddWhitelisted.signAsync(delegateBob)]);

                const whitelistedValidatorInfo = await polkadotJs.query.externalValidators.whitelistedValidators();
                expect(whitelistedValidatorInfo.toHuman().includes(delegateBob.address)).to.be.true;
            },
        });

        it({
            id: "E02",
            title: "Delegated account can sudo txs in external validator slashes",
            test: async function () {
                const txAddWhitelisted = polkadotJs.tx.proxy.proxy(
                    sudoAlice.address,
                    null,
                    polkadotJs.tx.sudo.sudo(
                        polkadotJs.tx.externalValidatorSlashes.forceInjectSlash(0, sudoAlice.address, 1000)
                    )
                );
                await context.createBlock([await txAddWhitelisted.signAsync(delegateBob)]);

                const DeferPeriod = (await polkadotJs.consts.externalValidatorSlashes.slashDeferDuration).toNumber();

                // scheduled slashes
                const expectedSlashes = await polkadotJs.query.externalValidatorSlashes.slashes(DeferPeriod + 1);
                expect(expectedSlashes.length).to.be.eq(1);
            },
        });
    },
});
