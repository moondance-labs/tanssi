import { expect, beforeAll, describeSuite } from "@moonwall/cli";
import { jumpSessions } from "../../../util/block";

describeSuite({
    id: "CT0406",
    title: "Configuration - ActiveConfig - targetContainerChainFullness",
    foundationMethods: "dev",
    testCases: ({ context, it }) => {
        beforeAll(async function () {
            const config = await context.polkadotJs().query.configuration.activeConfig();
            expect(config["targetContainerChainFullness"].toString()).toBe("800000000");

            const { result } = await context.createBlock(
                context
                    .polkadotJs()
                    .tx.sudo.sudo(context.polkadotJs().tx.configuration.setTargetContainerChainFullness(500000000n))
                    .signAsync(context.keyring.alice)
            );
            expect(result!.successful, result!.error?.name).to.be.true;

            await jumpSessions(context, 2);
        });

        it({
            id: "T01",
            title: "should set target fullness after 2 sessions",
            test: async function () {
                const config = await context.polkadotJs().query.configuration.activeConfig();
                expect(config["targetContainerChainFullness"].toString()).toBe("500000000");
            },
        });
    },
});
