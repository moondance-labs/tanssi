import { MoonwallContext, beforeAll, describeSuite, expect } from "@moonwall/cli";
import { generateKeyringPair } from "@moonwall/util";
import type { ApiPromise } from "@polkadot/api";

const MAX_BALANCE_TRANSFER_TRIES = 5;
describeSuite({
    id: "RT01",
    title: "Chopsticks Dancebox Upgrade Test",
    foundationMethods: "chopsticks",
    testCases: ({ it, context, log }) => {
        let api: ApiPromise;

        beforeAll(async () => {
            api = context.pjsApi
            
            console.dir( (await api.call.tanssiUtilApi.sessionPeriod()).toNumber(), {depth: 1})
          
            const specName = api.consts.system.version.specName.toString();
            const specVersion = getSpecVersion(api);
            log(`Currently connected to chain: ${specName} : ${specVersion}`);
        });

        it({
            id: "T1",
            timeout: 60000,
            title: "Can upgrade runtime",
            test: async () => {
                const rtBefore = getSpecVersion(api)
                const sessionBefore = api.query.session.currentIndex();
                log("About to upgrade to runtime at:");
                log((await MoonwallContext.getContext()).rtUpgradePath);
    
                await context.upgradeRuntime();
                const sessionAfter = api.query.session.currentIndex();
    
                // New sessions can lead to the runtime upgrade not being correctly applied
                // Hence we retry once more just in case
                if ((await sessionAfter).toNumber() > (await sessionBefore).toNumber() && rtBefore === getSpecVersion(api)) {
                    log("New session encountered, just in case retrying");
                    await context.upgradeRuntime();
                }

                // console.log( api.call.tanssiUtilApi)
    
                const rtafter = getSpecVersion(api)

                expect(rtBefore, `RT Upgrade has not been applied, before: ${rtBefore}, after: ${rtafter}`).not.toBe(rtafter);
            },           
        });

        it({
            id: "T2",
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
            id: "T3",
            timeout: 60000,
            title: "Can send balance transfers",
            test: async () => {
                const randomAccount = generateKeyringPair("sr25519");
                const keyring = new Keyring({ type: "sr25519" });
                const alice = keyring.addFromUri("//Alice", { name: "Alice default" });

                let tries = 0;
                const balanceBefore = (await api.query.system.account(randomAccount.address)).data.free.toBigInt();

                /// It might happen that by accident we hit a session change
                /// A block in which a session change occurs cannot hold any tx
                /// Chopsticks does not have the notion of tx pool either, so we need to retry
                /// Therefore we just retry at most MAX_BALANCE_TRANSFER_TRIES
                while (tries < MAX_BALANCE_TRANSFER_TRIES) {
                    const txHash = await api.tx.balances
                        .transferAllowDeath(randomAccount.address, 1_000_000_000)
                        .signAndSend(alice);
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


const getSpecVersion = (api: ApiPromise) => {
    return api.consts.system.version.specVersion.toNumber();
}
