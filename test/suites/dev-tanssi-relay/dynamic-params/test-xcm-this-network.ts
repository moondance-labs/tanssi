import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import type { KeyringPair } from "@moonwall/util";
import { ApiPromise } from "@polkadot/api";
import { DANCELIGHT_GENESIS_HASH, TANSSI_GENESIS_HASH } from "../../../utils/constants";

describeSuite({
    id: "DEVT1201",
    title: "Dynamic Parameters - XCM ThisNetwork Configuration",
    foundationMethods: "dev",
    testCases: ({ context, it }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let bob: KeyringPair;
        let dancelightNetworkId: any;
        let starlightNetworkId: any;
        let isStarlight: boolean;
        let networkId: any;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            alice = context.keyring.alice;
            bob = context.keyring.bob;

            dancelightNetworkId = {
                ByGenesis: DANCELIGHT_GENESIS_HASH,
            };

            starlightNetworkId = {
                ByGenesis: TANSSI_GENESIS_HASH,
            };

            const runtimeName = polkadotJs.runtimeVersion.specName.toString();
            isStarlight = runtimeName === "starlight";

            networkId = isStarlight ? starlightNetworkId : dancelightNetworkId;
        });

        it({
            id: "T01",
            title: "should allow root to update ThisNetwork parameter",
            test: async () => {
                const runtimeParameters = {
                    XcmConfig: {
                        ThisNetwork: [null, networkId],
                    },
                };

                // Set the parameter using sudo
                await context.createBlock(
                    context
                        .polkadotJs()
                        .tx.sudo.sudo(polkadotJs.tx.parameters.setParameter(runtimeParameters))
                        .signAsync(alice),
                    { allowFailures: false }
                );

                const onChainParameters = await polkadotJs.query.parameters.parameters({ XcmConfig: "ThisNetwork" });

                const expectedParameters = {
                    XcmConfig: {
                        ThisNetwork: networkId,
                    },
                };

                expect(onChainParameters.isSome).toBe(true);
                expect(onChainParameters.toHuman()).toEqual(expectedParameters);
            },
        });

        it({
            id: "T02",
            title: "should reject non-root attempts to update ThisNetwork parameter",
            test: async () => {
                const runtimeParameters = {
                    XcmConfig: {
                        ThisNetwork: [null, networkId],
                    },
                };

                // Try to set the parameter using non-root account (Bob)
                const tx = polkadotJs.tx.parameters.setParameter(runtimeParameters);
                const { result } = await context.createBlock(tx.signAsync(bob), { allowFailures: true });

                expect(result.successful).toBe(false);
            },
        });

        it({
            id: "T03",
            title: "should allow resetting ThisNetwork parameter",
            test: async () => {
                const onChainParametersBeforeReset = await polkadotJs.query.parameters.parameters({
                    XcmConfig: "ThisNetwork",
                });
                const expectedParametersBeforeReset = {
                    XcmConfig: {
                        ThisNetwork: networkId,
                    },
                };

                expect(onChainParametersBeforeReset.isSome).toBe(true);
                expect(onChainParametersBeforeReset.toHuman()).toEqual(expectedParametersBeforeReset);

                // Now reset to default by passing null/None
                const emptyRuntimeParameters = {
                    XcmConfig: {
                        ThisNetwork: [null, null],
                    },
                };

                await context.createBlock(
                    context
                        .polkadotJs()
                        .tx.sudo.sudo(polkadotJs.tx.parameters.setParameter(emptyRuntimeParameters))
                        .signAsync(alice),
                    { allowFailures: false }
                );

                // Verify the parameter was reset (should be None, meaning it uses default)
                const onChainParametersAfterReset = await polkadotJs.query.parameters.parameters({
                    XcmConfig: "ThisNetwork",
                });
                expect(onChainParametersAfterReset.isNone).toBe(true);
            },
        });
    },
});
