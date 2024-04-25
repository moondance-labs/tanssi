import { expect, describeSuite } from "@moonwall/cli";

describeSuite({
    id: "DF1001",
    title: "AuthoritiesNoting - OrchestratorParaId",
    foundationMethods: "dev",
    testCases: ({ context, it }) => {
        it({
            id: "T01",
            title: "should not set storage item if not sudo",
            test: async function () {
                const orchestratorParaId = await context.polkadotJs().query.authoritiesNoting.orchestratorParaId();
                expect(orchestratorParaId.toString()).toBe("1000");

                const { result } = await context.createBlock(
                    context.polkadotJs().tx.authoritiesNoting.setOrchestratorParaId(2000),
                    { allowFailures: true }
                );

                expect(result.successful).toBe(false);

                const newOrchestratorParaId = await context.polkadotJs().query.authoritiesNoting.orchestratorParaId();
                expect(newOrchestratorParaId.toString()).toBe("1000");
            },
        });

        it({
            id: "T02",
            title: "should set storage item via sudo",
            test: async function () {
                const orchestratorParaId = await context.polkadotJs().query.authoritiesNoting.orchestratorParaId();
                expect(orchestratorParaId.toString()).toBe("1000");

                await context.createBlock(
                    context
                        .polkadotJs()
                        .tx.sudo.sudo(context.polkadotJs().tx.authoritiesNoting.setOrchestratorParaId(2000))
                );

                const newOrchestratorParaId = await context.polkadotJs().query.authoritiesNoting.orchestratorParaId();
                expect(newOrchestratorParaId.toString()).toBe("2000");
            },
        });
    },
});
