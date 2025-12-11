import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { type KeyringPair, alith, baltathar } from "@moonwall/util";
import { Keyring } from "@polkadot/api";
import type { ApiPromise } from "@polkadot/api";
import type { Option } from "@polkadot/types";
import type { Codec } from "@polkadot/types/types";
import { DANCELIGHT_GENESIS_HASH, TANSSI_GENESIS_HASH } from "../../../utils/constants";

describeSuite({
    id: "COM0301",
    title: "Dynamic Parameters - RelayNetwork Configuration",
    foundationMethods: "dev",
    testCases: ({ context, it }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let bob: KeyringPair;
        let chain: string;
        let dancelightNetworkId: any;
        let starlightNetworkId: any;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            chain = polkadotJs.consts.system.version.specName.toString();
            alice =
                chain === "frontier-template"
                    ? alith
                    : new Keyring({ type: "sr25519" }).addFromUri("//Alice", {
                          name: "Alice default",
                      });
            bob =
                chain === "frontier-template"
                    ? baltathar
                    : new Keyring({ type: "sr25519" }).addFromUri("//Bob", {
                          name: "Bob default",
                      });

            dancelightNetworkId = {
                ByGenesis: DANCELIGHT_GENESIS_HASH,
            };

            starlightNetworkId = {
                ByGenesis: TANSSI_GENESIS_HASH,
            };
        });

        it({
            id: "T01",
            title: "should allow root to update RelayNetwork parameter",
            test: async () => {
                const runtimeParameters = {
                    XcmConfig: {
                        RelayNetwork: [null, dancelightNetworkId],
                    },
                };

                // Set the parameter using sudo
                await context.createBlock(
                    context
                        .polkadotJs()
                        .tx.sudo.sudo(polkadotJs.tx.parameters.setParameter(runtimeParameters as unknown as string))
                        .signAsync(alice),
                    { allowFailures: false }
                );

                const onChainParameters = (await polkadotJs.query.parameters.parameters({
                    XcmConfig: "RelayNetwork",
                })) as unknown as Option<Codec>;

                const expectedParameters = {
                    XcmConfig: {
                        RelayNetwork: dancelightNetworkId,
                    },
                };

                expect(onChainParameters.isSome).toBe(true);
                expect(onChainParameters.toHuman()).toEqual(expectedParameters);

                const updatedRuntimeParameters = {
                    XcmConfig: {
                        RelayNetwork: [null, starlightNetworkId],
                    },
                };

                await context.createBlock(
                    context
                        .polkadotJs()
                        .tx.sudo.sudo(
                            polkadotJs.tx.parameters.setParameter(updatedRuntimeParameters as unknown as string)
                        )
                        .signAsync(alice),
                    { allowFailures: false }
                );

                const onChainParametersAfterUpdate = (await polkadotJs.query.parameters.parameters({
                    XcmConfig: "RelayNetwork",
                })) as unknown as Option<Codec>;

                const expectedParametersAfterUpdate = {
                    XcmConfig: {
                        RelayNetwork: starlightNetworkId,
                    },
                };
                expect(onChainParametersAfterUpdate.isSome).toBe(true);
                expect(onChainParametersAfterUpdate.toHuman()).toEqual(expectedParametersAfterUpdate);
            },
        });

        it({
            id: "T02",
            title: "should reject non-root attempts to update RelayNetwork parameter",
            test: async () => {
                const runtimeParameters = {
                    XcmConfig: {
                        RelayNetwork: [null, dancelightNetworkId],
                    },
                };

                // Try to set the parameter using non-root account (Bob)
                const tx = polkadotJs.tx.parameters.setParameter(runtimeParameters as unknown as string);
                const { result } = await context.createBlock(tx.signAsync(bob), { allowFailures: true });

                expect(result.successful).toBe(false);
            },
        });

        it({
            id: "T03",
            title: "should allow resetting RelayNetwork parameter",
            test: async () => {
                const onChainParametersBeforeReset = (await polkadotJs.query.parameters.parameters({
                    XcmConfig: "RelayNetwork",
                })) as unknown as Option<Codec>;
                const expectedParametersBeforeReset = {
                    XcmConfig: {
                        RelayNetwork: starlightNetworkId,
                    },
                };

                expect(onChainParametersBeforeReset.isSome).toBe(true);
                expect(onChainParametersBeforeReset.toHuman()).toEqual(expectedParametersBeforeReset);

                // Now reset to default by passing null/None
                const emptyRuntimeParameters = {
                    XcmConfig: {
                        RelayNetwork: [null, null],
                    },
                };

                await context.createBlock(
                    context
                        .polkadotJs()
                        .tx.sudo.sudo(
                            polkadotJs.tx.parameters.setParameter(emptyRuntimeParameters as unknown as string)
                        )
                        .signAsync(alice),
                    { allowFailures: false }
                );

                // Verify the parameter was reset (should be None, meaning it uses default value)
                const onChainParametersAfterReset = (await polkadotJs.query.parameters.parameters({
                    XcmConfig: "RelayNetwork",
                })) as unknown as Option<Codec>;
                expect(onChainParametersAfterReset.isNone).toBe(true);
            },
        });
    },
});
