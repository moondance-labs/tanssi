import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { ApiPromise } from "@polkadot/api";

describeSuite({
    id: "DEVT0201",
    title: "Tanssi Relay Chain Spec",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;

        beforeAll(() => {
            polkadotJs = context.polkadotJs();
        });

        it({
            id: "E01",
            title: "Checking runtime version",
            test: async () => {
                const specName = polkadotJs.consts.system.version.specName.toString();
                const allowedSpecNames = ["dancelight", "starlight"];
                expect(allowedSpecNames.includes(specName), "Relay API incorrect").to.be.true;

                const authoringVersion = polkadotJs.consts.system.version.authoringVersion.toNumber();
                expect(authoringVersion, "Relay API incorrect").to.toBe(0);

                const implName = polkadotJs.consts.system.version.implName.toString();
                const allowedImplNames = ["tanssi-dancelight-v2.0", "tanssi-starlight-v2.0"];
                expect(allowedImplNames.includes(implName), "Relay API incorrect").to.be.true;

                const implVersion = polkadotJs.consts.system.version.implVersion.toNumber();
                expect(implVersion, "Relay API incorrect").to.toBe(0);

                const transactionVersion = polkadotJs.consts.system.version.transactionVersion.toNumber();
                expect(transactionVersion, "Relay API incorrect").to.toBe(26);

                const systemVersion = polkadotJs.consts.system.version.systemVersion.toNumber();
                expect(systemVersion, "Relay API incorrect").to.toBe(1);
            },
        });

        it({
            id: "E02",
            title: "Checking ss58 Prefix",
            test: async () => {
                const ss58Prefix = polkadotJs.consts.system.ss58Prefix.toNumber();
                expect(ss58Prefix, "Relay API incorrect").toBe(42);
            },
        });
    },
});
