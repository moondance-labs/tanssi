import { expect, beforeAll, describeSuite } from "@moonwall/cli";
import { jumpSessions } from "../../../util/block";

describeSuite({
    id: "CT0401",
    title: "Configuration - ActiveConfig - CollatorsPerContainer",
    foundationMethods: "dev",
    testCases: ({ context, it }) => {
        beforeAll(async function () {
            const config = await context.polkadotJs().query.configuration.activeConfig();
            expect(config["collatorsPerContainer"].toString()).toBe("2");

            const { result } = await context.createBlock(
                context
                    .polkadotJs()
                    .tx.sudo.sudo(context.polkadotJs().tx.configuration.setCollatorsPerContainer(5))
                    .signAsync(context.keyring.alice)
            );
            expect(result!.successful, result!.error?.name).to.be.true;

            await jumpSessions(context, 2);
        });

        it({
            id: "T01",
            title: "should set collators per container after 2 sessions",
            test: async function () {
                const config = await context.polkadotJs().query.configuration.activeConfig();
                expect(config["collatorsPerContainer"].toString()).toBe("5");
            },
        });
    },
});
