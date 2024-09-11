import { expect, beforeAll, describeSuite } from "@moonwall/cli";
import { jumpSessions } from "../../../util/block";

describeSuite({
    id: "CT0403",
    title: "Configuration - ActiveConfig - MaxOrchestratorCollators",
    foundationMethods: "dev",
    testCases: ({ context, it }) => {
        beforeAll(async function () {
            const config = await context.polkadotJs().query.collatorConfiguration.activeConfig();
            console.log(config.toJSON());
            expect(config["maxOrchestratorCollators"].toString()).toBe("0");

            const { result } = await context.createBlock(
                context
                    .polkadotJs()
                    .tx.sudo.sudo(context.polkadotJs().tx.collatorConfiguration.setMaxOrchestratorCollators(2))
                    .signAsync(context.keyring.alice)
            );
            expect(result!.successful, result!.error?.name).to.be.true;

            await jumpSessions(context, 2);
        });

        it({
            id: "T01",
            title: "should set max orchestrator collators after 2 sessions",
            test: async function () {
                const config = await context.polkadotJs().query.collatorConfiguration.activeConfig();
                expect(config["maxOrchestratorCollators"].toString()).toBe("2");
            },
        });
    },
});
