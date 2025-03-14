import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { KeyringPair } from "@moonwall/util";
import type { ApiPromise } from "@polkadot/api";

describeSuite({
    id: "DEVT1301",
    title: "Genesis supply and balances",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;

        beforeAll(() => {
            alice = context.keyring.alice;
            polkadotJs = context.polkadotJs();
        });

        it({
            id: "E01",
            title: "Checking total issuance is correct on genesis",
            test: async () => {
                const totalIssuance = (await polkadotJs.query.balances.totalIssuance()).toBigInt();
                expect(totalIssuance).toBe(12_000_000_000_199_999_998n);
            },
        });

        it({
            id: "E02",
            title: "Checking alice's balance is correct on genesis",
            test: async () => {
                const balance = (await polkadotJs.query.system.account(alice.address)).data.free.toBigInt();
                expect(balance).toBe(1_000_000_000_000_000_000n);
            },
        });
    },
});
