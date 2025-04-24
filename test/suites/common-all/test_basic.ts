import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { KeyringPair } from "@moonwall/util";
import type { ApiPromise } from "@polkadot/api";
import { STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_BALANCES, checkCallIsFiltered } from "helpers";

describeSuite({
    id: "C01",
    title: "Dev test suite",
    foundationMethods: "dev",
    testCases: ({ it, context, log }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let bob: KeyringPair;
        let chain: string;
        let isStarlight: boolean;
        let specVersion: number;
        let shouldSkipStarlightBalances: boolean;

        beforeAll(() => {
            polkadotJs = context.pjsApi;
            log(`This chain is ${context.isEthereumChain ? "Ethereum" : "Substrate"}`);
            alice = context.keyring.alice;
            bob = context.keyring.bob;

            chain = polkadotJs.consts.system.version.specName.toString();
            isStarlight = chain === "starlight";
            specVersion = polkadotJs.consts.system.version.specVersion.toNumber();
            shouldSkipStarlightBalances =
                isStarlight && STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_BALANCES.includes(specVersion);
        });

        it({
            id: "E01",
            title: "Checking that launched node can create blocks",
            test: async () => {
                const block = (await polkadotJs.rpc.chain.getBlock()).block.header.number.toNumber();
                await context.createBlock();

                const block2 = (await polkadotJs.rpc.chain.getBlock()).block.header.number.toNumber();
                log(`Original block #${block}, new block #${block2}`);
                expect(block2).to.be.greaterThan(block);
            },
        });

        it({
            id: "E02",
            title: "Checking that substrate txns possible",
            timeout: 20000,
            test: async () => {
                const balanceBefore = (await polkadotJs.query.system.account(bob.address)).data.free;

                const tx = polkadotJs.tx.balances.transferAllowDeath(bob.address, 1000);

                if (shouldSkipStarlightBalances) {
                    console.log(`Skipping E02 test for Starlight version ${specVersion}`);
                    await checkCallIsFiltered(context, polkadotJs, await tx.signAsync(alice));
                    return;
                }

                await tx.signAndSend(alice);

                await context.createBlock();
                const balanceAfter = (await polkadotJs.query.system.account(bob.address)).data.free;

                expect(balanceBefore.lt(balanceAfter)).to.be.true;
            },
        });

        it({
            id: "E03",
            title: "Checking that sudo can be used",
            test: async () => {
                await context.createBlock();
                const tx = polkadotJs.tx.rootTesting.fillBlock(60 * 10 ** 7);
                await polkadotJs.tx.sudo.sudo(tx).signAndSend(alice);

                await context.createBlock();
                const blockFill = await polkadotJs.query.system.blockWeight();
                expect(blockFill.normal.refTime.unwrap().toBigInt()).toBeGreaterThan(0n);
            },
        });
    },
});
