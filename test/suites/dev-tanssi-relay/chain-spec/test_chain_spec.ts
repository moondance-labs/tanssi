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
            title: "Checking runtime version",
            test: async function () {
                const specName = polkadotJs.consts.system.version.specName.toString();
                expect(specName, "Relay API incorrect").to.toBe("starlight");

                const specVersion = polkadotJs.consts.system.version.specVersion.toNumber();
                expect(specVersion, "Relay API incorrect").toBe(1_011_000);

                const authoringVersion = polkadotJs.consts.system.version.authoringVersion.toNumber();
                expect(authoringVersion, "Relay API incorrect").to.toBe(0);

                const implName = polkadotJs.consts.system.version.implName.toString();
                expect(implName, "Relay API incorrect").to.toBe("tanssi-starlight-v2.0");

                const implVersion = polkadotJs.consts.system.version.implVersion.toNumber();
                expect(implVersion, "Relay API incorrect").to.toBe(0);

                const transactionVersion = polkadotJs.consts.system.version.transactionVersion.toNumber();
                expect(transactionVersion, "Relay API incorrect").to.toBe(25);

                const stateVersion = polkadotJs.consts.system.version.stateVersion.toNumber();
                expect(stateVersion, "Relay API incorrect").to.toBe(1);
            },
        });

        it({
            id: "E02",
            title: "Checking ss58 Prefix",
            test: async function () {
                const ss58Prefix = polkadotJs.consts.system.ss58Prefix.toNumber();
                expect(ss58Prefix, "Relay API incorrect").toBe(42);
            },
        });
    },
});
