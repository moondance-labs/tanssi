import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { ApiPromise } from "@polkadot/api";
import type { ApiDecoration } from "@polkadot/api/types";

describeSuite({
    id: "SMO05",
    title: "Verify treasury consistency",
    foundationMethods: "read_only",
    testCases: ({ context, it, log }) => {
        let atBlockNumber = 0;
        let apiAt: ApiDecoration<"promise">;
        let paraApi: ApiPromise;

        beforeAll(async () => {
            paraApi = context.polkadotJs("para");
            atBlockNumber = (await paraApi.rpc.chain.getHeader()).number.toNumber();
            apiAt = await paraApi.at(await paraApi.rpc.chain.getBlockHash(atBlockNumber));
        });

        it({
            id: "C100",
            title: "should have value > 0",
            test: async () => {
                // Load data
                const treasuryPalletId = paraApi.consts.treasury.palletId;
                const treasuryAccount = await apiAt.query.system.account(
                    `0x6d6f646C${treasuryPalletId.toString().slice(2)}0000000000000000000000000000000000000000`
                );

                console.log(
                    `0x6d6f646C${treasuryPalletId.toString().slice(2)}0000000000000000000000000000000000000000`
                );
                expect(treasuryAccount.data.free.toBigInt() > 0n).to.be.true;
                expect(treasuryAccount.data.reserved.toBigInt()).to.be.equal(0n);

                log("Verified treasury free/reserved balance");
            },
        });
    },
});
