import { expect, describeSuite } from "@moonwall/cli";

describeSuite({
    id: "CT0405",
    title: "Configuration - ActiveConfig - Origin",
    foundationMethods: "dev",
    testCases: ({ context, it }) => {
        it({
            id: "T01",
            title: "should fail on setMaxCollators if not sudo",
            test: async function () {
                const { result } = await context.createBlock(
                    context.polkadotJs().tx.configuration.setMaxCollators(200).signAsync(context.keyring.bob),
                    { allowFailures: true }
                );

                expect(result.successful).toBe(false);
            },
        });

        it({
            id: "T02",
            title: "should fail on setMinOrchestratorCollators if not sudo",
            test: async function () {
                const { result } = await context.createBlock(
                    context.polkadotJs().tx.configuration.setMinOrchestratorCollators(2).signAsync(context.keyring.bob),
                    { allowFailures: true }
                );

                expect(result.successful).toBe(false);
            },
        });

        it({
            id: "T03",
            title: "should fail on setMaxOrchestratorCollators if not sudo",
            test: async function () {
                const { result } = await context.createBlock(
                    context.polkadotJs().tx.configuration.setMaxOrchestratorCollators(2).signAsync(context.keyring.bob),
                    { allowFailures: true }
                );

                expect(result.successful).toBe(false);
            },
        });

        it({
            id: "T04",
            title: "should fail on setCollatorsPerContainer if not sudo",
            test: async function () {
                const { result } = await context.createBlock(
                    context.polkadotJs().tx.configuration.setCollatorsPerContainer(5).signAsync(context.keyring.bob),
                    { allowFailures: true }
                );

                expect(result.successful).toBe(false);
            },
        });
    },
});
