import "@polkadot/api-augment";
import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { KeyringPair } from "@moonwall/util";
import { ApiPromise } from "@polkadot/api";

describeSuite({
    id: "DTR0001",
    title: "Dev test suite",
    foundationMethods: "dev",
    testCases: ({ it, context, log }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let bob: KeyringPair;
        let charlie: KeyringPair;

        beforeAll(() => {
            polkadotJs = context.pjsApi;
            log(`This chain is ${context.isEthereumChain ? "Ethereum" : "Substrate"}`);
            alice = context.keyring.alice;
            bob = context.keyring.bob;
            charlie = context.keyring.charlie;
        });

        it({
            id: "E01",
            title: "Checking that launched node can create blocks",
            test: async function () {
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
            timeout: 200000,
            test: async function () {
                const balanceBefore = (await polkadotJs.query.system.account(bob.address)).data.free.toBigInt();

                await polkadotJs.tx.balances.transferAllowDeath(bob.address, 1000).signAndSend(charlie);

                await context.createBlock();
                const balanceAfter = (await polkadotJs.query.system.account(bob.address)).data.free.toBigInt();

                expect(balanceAfter).to.be.equal(balanceBefore + 1000n);
            },
        });

        it({
            id: "E03",
            title: "Checking that sudo can be used",
            timeout: 200000,
            test: async function () {
                await context.createBlock();
                //const tx = polkadotJs.tx.rootTesting.fillBlock(60 * 10 ** 7);
                await polkadotJs.tx.sudo
                    .sudo(polkadotJs.tx.balances.transferAllowDeath(bob.address, 1000))
                    .signAndSend(alice);

                await context.createBlock();
                const blockFill = await polkadotJs.query.system.blockWeight();
                expect(blockFill.normal.refTime.unwrap().toBigInt()).toBeGreaterThan(0n);
            },
        });
    },
});
