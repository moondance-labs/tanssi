import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import type { ApiPromise } from "@polkadot/api";
import { jumpToSession } from "../../../util/block";

describeSuite({
    id: "DTR1202",
    title: "BEEFY - Set new genesis",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        beforeAll(() => {
            polkadotJs = context.polkadotJs();
        });

        it({
            id: "E01",
            title: "Should be able to update the genesis BEEFY block",
            test: async () => {
                await jumpToSession(context, 1);

                const genesisDelayBefore = (await polkadotJs.query.beefy.genesisBlock()).toHuman();
                expect(Number(genesisDelayBefore)).to.eq(1);

                const newGenesisDelay: number = 5;
                const { result } = await context.createBlock(
                    polkadotJs.tx.sudo
                        .sudo(polkadotJs.tx.beefy.setNewGenesis(newGenesisDelay))
                        .signAsync(context.keyring.alice),
                    { allowFailures: false }
                );
                expect(result!.successful, result!.error?.name).to.be.true;

                const currentBlockNumber = (await polkadotJs.rpc.chain.getHeader()).number.toNumber();
                const genesisDelayAfter = (await polkadotJs.query.beefy.genesisBlock()).toHuman();
                expect(genesisDelayAfter).to.eq((currentBlockNumber + newGenesisDelay).toString());
            },
        });
    },
});
