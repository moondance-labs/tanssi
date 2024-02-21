import { MoonwallContext, beforeAll, describeSuite, expect } from "@moonwall/cli";
import { generateKeyringPair } from "@moonwall/util";
import { ApiPromise } from "@polkadot/api";
import { alith } from "@moonwall/util";

const MAX_BALANCE_TRANSFER_TRIES = 5;
describeSuite({
    id: "CAN",
    title: "Chopsticks Frontier Template Upgrade Test",
    foundationMethods: "chopsticks",
    testCases: function ({ it, context, log }) {
        let api: ApiPromise;

        beforeAll(async () => {
            api = context.polkadotJs();

            const rtBefore = api.consts.system.version.specVersion.toNumber();
            log(`About to upgrade to runtime at:`);
            log((await MoonwallContext.getContext()).rtUpgradePath);

            await context.upgradeRuntime();

            const rtafter = api.consts.system.version.specVersion.toNumber();

            if (rtBefore === rtafter) {
                throw new Error("Runtime upgrade failed");
            }

            log(`RT upgrade has increased specVersion from ${rtBefore} to ${rtafter}`);

            const specName = api.consts.system.version.specName.toString();
            log(`Currently connected to chain: ${specName}`);
        });

        it({
            id: "T1",
            timeout: 60000,
            title: "Can create new blocks",
            test: async () => {
                const currentHeight = (await api.rpc.chain.getBlock()).block.header.number.toNumber();
                await context.createBlock({ count: 2 });
                const newHeight = (await api.rpc.chain.getBlock()).block.header.number.toNumber();
                expect(newHeight - currentHeight).to.be.equal(2);
            },
        });
        it({
            id: "T2",
            timeout: 60000,
            title: "Can send balance transfers",
            test: async () => {
                const randomAccount = generateKeyringPair("ethereum");

                let tries = 0;
                const balanceBefore = (await api.query.system.account(randomAccount.address)).data.free.toBigInt();

                /// It might happen that by accident we hit a session change
                /// A block in which a session change occurs cannot hold any tx
                /// Chopsticks does not have the notion of tx pool either, so we need to retry
                /// Therefore we just retry at most MAX_BALANCE_TRANSFER_TRIES
                while (tries < MAX_BALANCE_TRANSFER_TRIES) {
                    const txHash = await api.tx.balances
                        .transferAllowDeath(randomAccount.address, 1_000_000_000)
                        .signAndSend(alith);
                    const result = await context.createBlock({ count: 1 });

                    const block = await api.rpc.chain.getBlock(result.result);
                    const includedTxHashes = block.block.extrinsics.map((x) => x.hash.toString());
                    if (includedTxHashes.includes(txHash.toString())) {
                        break;
                    }
                    tries++;
                }

                const balanceAfter = (await api.query.system.account(randomAccount.address)).data.free.toBigInt();
                expect(balanceBefore < balanceAfter).to.be.true;
            },
        });
    },
});
