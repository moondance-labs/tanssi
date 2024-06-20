import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { ApiPromise } from "@polkadot/api";
import { hexToString } from "@polkadot/util";

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
            title: "Checking spec details",
            test: async function () {
                const specName = polkadotJs.consts.system.version.specName.toString();
                expect(specName, "Relay API incorrect").to.contain("mozart");

                const specVersion = polkadotJs.consts.system.version.specVersion.toString();
                expect(specVersion, "Relay API incorrect").toBe("1011000");

                const ss58Prefix = polkadotJs.consts.system.ss58Prefix.toString();
                expect(ss58Prefix, "Relay API incorrect").toBe("42");
            },
        });
    },
});
