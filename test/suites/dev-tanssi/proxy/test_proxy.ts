import { describeSuite, expect, beforeAll} from "@moonwall/cli";
import { setupLogger } from "@moonwall/util";
import { ApiPromise, Keyring } from "@polkadot/api";

import "@polkadot/api-augment";

describeSuite({
  id: "D06",
  title: "Proxy test suite",
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
      title: "No proxies at genesis",
      test: async function () {
        await context.createBlock();
        const proxies = await polkadotJs.query.proxy.proxies(alice.address);
        expect(proxies.toJSON()[0]).to.deep.equal([]);
      },
    });

    it({
      id: "E02",
      title: "Add proxy",
      test: async function () {
        await context.createBlock();

        const delegate = bob.address;
        const tx = polkadotJs.tx.proxy.addProxy(delegate, 'Any', 0);
        await context.createBlock([
          await tx.signAsync(alice),
        ]);

        const proxies = await polkadotJs.query.proxy.proxies(alice.address);
        expect(proxies.toJSON()[0]).to.deep.equal([{
          delegate,
          proxyType: 'Any',
          delay: 0,
        }]);
      },
    });

    it({
      id: "E03",
      title: "Delegate account can call proxy.proxy",
      test: async function () {
        await context.createBlock();

        const balanceBefore = (await polkadotJs.query.system.account(bob.address)).data.free;
        const tx = polkadotJs.tx.proxy.proxy(alice.address, null, polkadotJs.tx.balances.transfer(bob.address, 200_000));
        await context.createBlock([
          await tx.signAsync(bob),
        ]);
        const balanceAfter = (await polkadotJs.query.system.account(bob.address)).data.free;

        // Balance of Bob account increased
        expect(balanceBefore.lt(balanceAfter)).to.be.true;
      },
    });
    },
});
