import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { ApiPromise } from "@polkadot/api";

describeSuite({
    id: "DTR0101",
    title: "Tanssi Relay Chain Spec",
    foundationMethods: "dev",
    testCases: function ({ it, context }) {
        let polkadotJs: ApiPromise;

        beforeAll(() => {
            polkadotJs = context.polkadotJs();
        });

        it({
            id: "E01",
            title: "Checking spec name and version",
            test: async function () {
                const relayNetwork = polkadotJs.consts.system;

                const specName = relayNetwork.version.specName.toString();
                expect(specName, "Relay API incorrect").to.contain("mozart");

                const specVersion = relayNetwork.version.specVersion.toString();
                expect(specVersion, "Relay API incorrect").to.contain("1011000");
            },
        });
    },
});
