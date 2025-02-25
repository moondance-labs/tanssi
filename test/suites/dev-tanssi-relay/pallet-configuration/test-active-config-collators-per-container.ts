import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { jumpSessions } from "utils";

describeSuite({
    id: "DEVT0801",
    title: "Configuration - ActiveConfig - CollatorsPerContainer",
    foundationMethods: "dev",
    testCases: ({ context, it }) => {
        beforeAll(async () => {
            const config = await context.polkadotJs().query.collatorConfiguration.activeConfig();
            expect(config.collatorsPerContainer.toString()).toBe("2");

            const { result } = await context.createBlock(
                context
                    .polkadotJs()
                    .tx.sudo.sudo(context.polkadotJs().tx.collatorConfiguration.setCollatorsPerContainer(5))
                    .signAsync(context.keyring.alice)
            );
            expect(result?.successful, result?.error?.name).to.be.true;

            await jumpSessions(context, 2);
        });

        it({
            id: "T01",
            title: "should set collators per container after 2 sessions",
            test: async () => {
                const config = await context.polkadotJs().query.collatorConfiguration.activeConfig();
                expect(config.collatorsPerContainer.toString()).toBe("5");
            },
        });
    },
});
