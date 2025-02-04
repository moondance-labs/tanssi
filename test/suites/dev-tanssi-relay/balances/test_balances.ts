import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import type { ApiPromise } from "@polkadot/api";
import type { KeyringPair } from "@moonwall/util";

describeSuite({
    id: "DTR0101",
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
                expect(totalIssuance).toBe(12_000_000_000_166_666_665n);
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
