import { describeSuite, expect, beforeAll} from "@moonwall/cli";
import { setupLogger } from "@moonwall/util";
import { ApiPromise, Keyring } from "@polkadot/api";
import "@polkadot/api-augment";
import { initializeCustomCreateBlock } from "../../../util/block";

describeSuite({
  id: "D08",
  title: "Proxy test suite - ProxyType::Balances",
  foundationMethods: "dev",
  testCases: ({ it, context, log }) => {
    let polkadotJs: ApiPromise;
    const anotherLogger = setupLogger("anotherLogger");
    let alice, bob, charlie, dave;
    initializeCustomCreateBlock(context);
    
    beforeAll(() => {
      const keyring = new Keyring({ type: 'sr25519' });
      alice = keyring.addFromUri('//Alice', { name: 'Alice default' });
      bob = keyring.addFromUri('//Bob', { name: 'Bob default' });
      charlie = keyring.addFromUri('//Charlie', { name: 'Charlie default' });
      dave = keyring.addFromUri('//Dave', { name: 'Dave default' });
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
      title: "Add proxy Balances",
      test: async function () {
        const delegate = charlie.address;
        const balances = 5;
        const delay = 0;
        const tx = polkadotJs.tx.proxy.addProxy(delegate, balances, delay);
        await context.createBlock([
          await tx.signAsync(alice),
        ]);

        const events = await polkadotJs.query.system.events();
        const ev1 = events.filter(
          (a) => {
            return a.event.method == "ProxyAdded";
        });
        expect(ev1.length).to.be.equal(1);

        const proxies = await polkadotJs.query.proxy.proxies(alice.address);
        expect(proxies.toJSON()[0]).to.deep.equal([{
          delegate: charlie.address,
          proxyType: 'Balances',
          delay: 0,
        }]);
      },
    });

    it({
      id: "E03",
      title: "Delegate account can call balance.transfer",
      test: async function () {
        await context.createBlock();

        const tx = polkadotJs.tx.proxy.proxy(alice.address, null, polkadotJs.tx.balances.transfer(charlie.address, 200_000));
        await context.createBlock([
          await tx.signAsync(charlie),
        ]);
        const events = await polkadotJs.query.system.events();
        const ev1 = events.filter(
          (a) => {
            return a.event.method == "ProxyExecuted";
        });
        expect(ev1.length).to.be.equal(1);
        expect(ev1[0].event.data[0].toString()).to.be.eq("Ok");
      },
    });

    it({
      id: "E04",
      title: "Delegate account cannot call system.remark",
      test: async function () {
        await context.createBlock();

        const tx = polkadotJs.tx.proxy.proxy(alice.address, null, polkadotJs.tx.system.remarkWithEvent('I was called through using proxy.proxy'));
        await context.createBlock([
          await tx.signAsync(charlie),
        ]);
        const events = await polkadotJs.query.system.events();
        const ev1 = events.filter(
          (a) => {
            return a.event.method == "ProxyExecuted";
        });
        expect(ev1.length).to.be.equal(1);
        expect(ev1[0].event.data[0].toString()).to.not.be.eq("Ok");

        const ev2 = events.filter(
          (a) => {
            return a.event.method == "Remarked";
        });
        expect(ev2.length).to.be.equal(0);
      },
    });
    },
});
