import { describeSuite, expect, beforeAll} from "@moonwall/cli";
import { setupLogger } from "@moonwall/util";
import { ApiPromise, Keyring } from "@polkadot/api";
import "@polkadot/api-augment";

describeSuite({
  id: "D02",
  title: "Registrar test suite",
  foundationMethods: "dev",
  testCases: ({ it, context, log }) => {
    let polkadotJs: ApiPromise;
    const anotherLogger = setupLogger("anotherLogger");
    let alice, bob;
    beforeAll(() => {
      const keyring = new Keyring({ type: 'sr25519' });
      alice = keyring.addFromUri('//Alice', { name: 'Alice default' });
      bob = keyring.addFromUri('//Bob', { name: 'Bob default' });
      polkadotJs = context.polkadotJs();
    });

    it({
        id: "E01",
        title: "Checking that fetching registered paraIds is possible",
        test: async function () {
            const parasRegistered = await polkadotJs.query.registrar.registeredParaIds();

            // The default genesis start empty
            // TODO: this does not work
            //expect(parasRegistered).to.be.eq([]);
            expect(parasRegistered[0]).to.be.eq(undefined);
        },
      });

    it({
      id: "E02",
      title: "Checking that registering paraIds is possible",
      test: async function () {
        await context.createBlock();

        const currentSesssion = await polkadotJs.query.session.currentIndex();
        const sessionDelay = await polkadotJs.consts.registrar.sessionDelay;
        const expectedScheduledOnboarding = BigInt(currentSesssion.toString()) + BigInt(sessionDelay.toString());
        const emptyGenesisData = () => {
            // TODO: fill with default value for all the entries of ContainerChainGenesisData
            let g = {
              id: "container-chain-2002",
              name: "Container Chain 2002",
            };
            return g;
        };
        const containerChainGenesisData = emptyGenesisData();

        const tx = polkadotJs.tx.registrar.register(2002, containerChainGenesisData);
        let txRes1 = await polkadotJs.tx.sudo.sudo(tx).signAndSend(alice);
        console.log(txRes1);

        await context.createBlock();

        const pendingParas = await polkadotJs.query.registrar.pendingParaIds();
        const sessionScheduling = pendingParas[0][0];
        const parasScheduled = pendingParas[0][1];

        expect(pendingParas.length).to.be.eq(1);
        expect(sessionScheduling.toBigInt()).to.be.eq(expectedScheduledOnboarding);
        expect(parasScheduled.length).to.be.eq(1);

        // These will be the paras in session 2
        expect(parasScheduled[0].toBigInt()).to.be.eq(BigInt(2002));
      },
    });
    },
});