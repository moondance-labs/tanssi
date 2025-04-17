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
        let specVersion: number;
        let specName: string;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            alice = context.keyring.alice;
            bob = context.keyring.bob;
            specVersion = polkadotJs.consts.system.version.specVersion.toNumber();
            specName = polkadotJs.consts.system.version.specName.toString();
        });

        it({
            id: "E01",
            title: "Balances calls are filtered",
            test: async () => {
                if (specName === "starlight" && specVersion >= 1300) {
                    const balanceBefore = (await polkadotJs.query.system.account(bob.address)).data.free;

                    await checkCallIsFiltered(context, polkadotJs, async () => {
                        await polkadotJs.tx.balances.transferAllowDeath(bob.address, 1000).signAsync(alice);
                    });

                    const balanceAfter = (await polkadotJs.query.system.account(bob.address)).data.free;
                    expect(balanceBefore.toBigInt()).to.be.eq(balanceAfter.toBigInt());
                }
            },
        });

        it({
            id: "E02",
            title: "Bridge calls are filtered",
            test: async () => {
                if (specName === "starlight" && specVersion >= 1300) {
                    await checkCallIsFiltered(context, polkadotJs, async () => {
                        await polkadotJs.tx.ethereumSystem.createAgent().signAsync(alice);
                    });

                    await checkCallIsFiltered(context, polkadotJs, async () => {
                        await polkadotJs.tx.ethereumOutboundQueue.setOperatingMode('Halted').signAsync(alice);
                    });

                    await checkCallIsFiltered(context, polkadotJs, async () => {
                        await polkadotJs.tx.ethereumInboundQueue.setOperatingMode('Halted').signAsync(alice);
                    });

                    await checkCallIsFiltered(context, polkadotJs, async () => {
                        await polkadotJs.tx.ethereumBeaconClient.setOperatingMode('Halted').signAsync(alice);
                    });

                    await checkCallIsFiltered(context, polkadotJs, async () => {
                        const recipient = "0x0000000000000000000000000000000000000001";
                        await polkadotJs.tx.ethereumTokenTransfers.transferNativeToken(12345n, recipient).signAsync(alice);
                    });
                }
            },
        });

        it({
            id: "E03",
            title: "Staking calls are filtered",
            test: async () => {
                if (specName === "starlight" && specVersion >= 1300) {
                    await checkCallIsFiltered(context, polkadotJs, async () => {
                        await polkadotJs.tx.pooledStaking.updateCandidatePosition([]).signAsync(alice);
                    });
                }
            },
        });
    },
});

async function checkCallIsFiltered(context: any, polkadotJs: ApiPromise, tx: () => Promise<void>): Promise<string[]> {
    try {
        await context.createBlock(await tx(), { allowFailures: false });
        // Si llegamos aquí es que no falló como esperábamos
        return [];
    } catch (e) {
        console.log(e);
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
}
