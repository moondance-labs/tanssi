import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { jumpSessions } from "utils";

describeSuite({
    id: "COMMO0302",
    title: "Configuration - ActiveConfig - MaxCollators",
    foundationMethods: "dev",
    testCases: ({ context, it }) => {
        beforeAll(async () => {
            const config = await context.polkadotJs().query.configuration.activeConfig();
            expect(config.maxCollators.toString()).toBe("100");

            const { result } = await context.createBlock(
                context
                    .polkadotJs()
                    .tx.sudo.sudo(context.polkadotJs().tx.configuration.setMaxCollators(200))
                    .signAsync(context.keyring.alice)
            );
            expect(result?.successful, result?.error?.name).to.be.true;

            await jumpSessions(context, 2);
        });

        it({
            id: "T01",
            title: "should set max collators after 2 sessions",
            test: async () => {
                const config = await context.polkadotJs().query.configuration.activeConfig();
                expect(config.maxCollators.toString()).toBe("200");
            },
        });
    },
});
