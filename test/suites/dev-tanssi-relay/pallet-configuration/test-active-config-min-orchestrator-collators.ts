import { expect, beforeAll, describeSuite } from "@moonwall/cli";
import { initializeCustomCreateBlock, jumpSessions } from "../../../util/block";

describeSuite({
    id: "CT0404",
    title: "Configuration - ActiveConfig - MinOrchestratorCollators",
    foundationMethods: "dev",
    testCases: ({ context, it }) => {
        beforeAll(async () => {
            initializeCustomCreateBlock(context);

            const config = await context.polkadotJs().query.collatorConfiguration.activeConfig();
            expect(config["minOrchestratorCollators"].toString()).toBe("0");

            const { result } = await context.createBlock(
                await context
                    .polkadotJs()
                    .tx.sudo.sudo(context.polkadotJs().tx.collatorConfiguration.setMinOrchestratorCollators(2))
                    .signAsync(context.keyring.alice)
            );
            expect(result!.successful, result!.error?.name).to.be.true;

            await jumpSessions(context, 2);
        });

        it({
            id: "T01",
            title: "should set max orchestrator collators after 2 sessions",
            test: async () => {
                const config = await context.polkadotJs().query.collatorConfiguration.activeConfig();
                expect(config["minOrchestratorCollators"].toString()).toBe("2");
            },
        });
    },
});
