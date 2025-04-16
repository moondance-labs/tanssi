import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { KeyringPair } from "@moonwall/util";
import { type ApiPromise } from "@polkadot/api";
import type { SpRuntimeDispatchError } from "@polkadot/types/lookup";

describeSuite({
    id: "DEVT2001",
    title: "Filter calls test",
    foundationMethods: "dev",

    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let bob: KeyringPair;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            alice = context.keyring.alice;
            bob = context.keyring.bob;
        });

        it({
            id: "E01",
            title: "Balances calls are filtered",
            test: async () => {
                const specVersion = polkadotJs.consts.system.version.specVersion.toNumber();
                if (specVersion >= 1300) {
                    const balanceBefore = (await polkadotJs.query.system.account(bob.address)).data.free;

                    const tx = polkadotJs.tx.balances.transferAllowDeath(bob.address, 1000);
                    try {
                        await context.createBlock(await tx.signAsync(alice), { allowFailures: false });
                    } catch(e) {
                        const events = await polkadotJs.query.system.events();
                        const errors = events
                        .filter(({ event }) => polkadotJs.events.system.ExtrinsicFailed.is(event))
                        .map(
                          ({
                            event: {
                              data: [error],
                            },
                          }) => {
                            const dispatchError = error as SpRuntimeDispatchError;
                            if (dispatchError.isModule) {
                              const decoded = polkadotJs.registry.findMetaError(dispatchError.asModule);
                              const { method } = decoded;
                
                              return `${method}`;
                            }
                            return error.toString();
                          }
                        );

                        expect(errors.length).to.be.eq(1);
                        expect(errors[0]).to.be.eq("CallFiltered");
                    }
                    
                    await context.createBlock();
                    const balanceAfter = (await polkadotJs.query.system.account(bob.address)).data.free;
    
                    expect(balanceBefore.toBigInt()).to.be.eq(balanceAfter.toBigInt());
                }
            },
        });
    },
});
