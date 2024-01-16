import { expect, beforeAll, describeSuite } from "@moonwall/cli";
import { initializeCustomCreateBlock, jumpSessions } from "../../../util/block";

describeSuite({
    id: "CT0404",
    title: "Configuration - ActiveConfig - MinOrchestratorCollators",
    foundationMethods: "dev",
    testCases: ({ context, it }) => {
        beforeAll(async function () {
            initializeCustomCreateBlock(context);

            const config = await context.polkadotJs().query.configuration.activeConfig();
            expect(config["minOrchestratorCollators"].toString()).toBe("1");

            const { result } = await context.createBlock(
                await context
                    .polkadotJs()
                    .tx.sudo.sudo(context.polkadotJs().tx.configuration.setMinOrchestratorCollators(2))
                    .signAsync(context.keyring.alice)
            );
            expect(result!.successful, result!.error?.name).to.be.true;

            await jumpSessions(context, 2);
        });

        it({
            id: "T01",
            title: "should set max orchestrator collators after 2 sessions",
            test: async function () {
                const config = await context.polkadotJs().query.configuration.activeConfig();
                expect(config["minOrchestratorCollators"].toString()).toBe("2");
            },
        });
    },
});
