import { expect, beforeAll, describeSuite } from "@moonwall/cli";
import { jumpSessions } from "../../../util/block";

describeSuite({
    id: "CT0403",
    title: "Configuration - ActiveConfig - MaxOrchestratorCollators",
    foundationMethods: "dev",
    testCases: ({ context, it }) => {
        beforeAll(async function () {
            const config = await context.polkadotJs().query.configuration.activeConfig();
            expect(config["maxOrchestratorCollators"].toString()).toBe("1");

            const { result } = await context.createBlock(
                context
                    .polkadotJs()
                    .tx.sudo.sudo(context.polkadotJs().tx.configuration.setMaxOrchestratorCollators(2))
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
                expect(config["maxOrchestratorCollators"].toString()).toBe("2");
            },
        });
    },
});
