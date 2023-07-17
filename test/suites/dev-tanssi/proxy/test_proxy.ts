import { describeSuite, expect, beforeAll} from "@moonwall/cli";
import { setupLogger } from "@moonwall/util";
import { ApiPromise, Keyring } from "@polkadot/api";
import "@polkadot/api-augment";
import { initializeCustomCreateBlock } from "../../../util/block";

describeSuite({
  id: "D06",
  title: "Proxy test suite",
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
      title: "Add proxy",
      test: async function () {
        await context.createBlock();

        const delegate = bob.address;
        const tx = polkadotJs.tx.proxy.addProxy(delegate, 'Any', 0);
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
        const balanceBefore = (await polkadotJs.query.system.account(bob.address)).data.free;
        const tx = polkadotJs.tx.proxy.proxy(alice.address, null, polkadotJs.tx.balances.transfer(bob.address, 200_000));
        await context.createBlock([
          await tx.signAsync(bob),
        ]);

        const events = await polkadotJs.query.system.events();
        const ev1 = events.filter(
          (a) => {
            return a.event.method == "ProxyExecuted";
        });
        expect(ev1.length).to.be.equal(1);
        expect(ev1[0].event.data[0].toString()).to.be.eq("Ok");

        const balanceAfter = (await polkadotJs.query.system.account(bob.address)).data.free;

        // Balance of Bob account increased
        expect(balanceBefore.lt(balanceAfter)).to.be.true;
      },
    });

    it({
      id: "E04",
      title: "Unauthorized account cannot call proxy.proxy",
      test: async function () {
        await context.createBlock();

        const balanceBefore = (await polkadotJs.query.system.account(charlie.address)).data.free;
        const tx = polkadotJs.tx.proxy.proxy(alice.address, null, polkadotJs.tx.balances.transfer(charlie.address, 200_000));
        await context.createBlock([
          await tx.signAsync(charlie),
        ]);

        const events = await polkadotJs.query.system.events();
        const ev1 = events.filter(
          (a) => {
            return a.event.method == "ExtrinsicFailed";
        });
        expect(ev1.length).to.be.equal(1);

        const balanceAfter = (await polkadotJs.query.system.account(charlie.address)).data.free;

        // Balance of Charlie account must be the same
        expect(balanceBefore.eq(balanceAfter)).to.be.true;
      },
    });

    it({
      id: "E05",
      title: "Can add multiple proxy types to the same delegator",
      test: async function () {
        await context.createBlock();

        const delegate = dave.address;
        const txs = [];
        // All proxy types that do not allow balance transfer
        const proxyTypes = [
          // NonTransfer = 1, Governance = 2, Staking = 3, CancelProxy = 4
          1, 2, 3, 4
        ];
        const nonce = await polkadotJs.rpc.system.accountNextIndex(alice.publicKey);
        for (let [i, proxyType] of proxyTypes.entries()) {
          const tx = polkadotJs.tx.proxy.addProxy(delegate, proxyType, 0);
          txs.push(await tx.signAsync(alice, { nonce: nonce.addn(i) }));
        }
        await context.createBlock(txs);

        const events = await polkadotJs.query.system.events();
        const ev1 = events.filter(
          (a) => {
            return a.event.method == "ProxyAdded";
        });
        expect(ev1.length).to.be.equal(proxyTypes.length);

        const proxies = await polkadotJs.query.proxy.proxies(alice.address);
        expect(proxies.toJSON()[0].length).to.be.equal(proxyTypes.length + 1);
      },
    });

    it({
      id: "E06",
      title: "Account with no balance proxy cannot call balances.transfer",
      test: async function () {
        // Dave has multiple proxy types, but none of them allows to call balances.transfer
        const balanceBefore = (await polkadotJs.query.system.account(dave.address)).data.free;
        const tx = polkadotJs.tx.proxy.proxy(alice.address, null, polkadotJs.tx.balances.transfer(dave.address, 200_000));
        await context.createBlock([
          await tx.signAsync(dave),
        ]);

        const events = await polkadotJs.query.system.events();
        const ev1 = events.filter(
          (a) => {
            return a.event.method == "ProxyExecuted";
        });
        expect(ev1.length).to.be.equal(1);
        expect(ev1[0].event.data[0].toString()).to.not.be.eq("Ok");

        const balanceAfter = (await polkadotJs.query.system.account(dave.address)).data.free;

        // Balance of Dave account must be the same
        expect(balanceBefore.eq(balanceAfter)).to.be.true;
      },
    });

    it({
      id: "E06",
      title: "Account with non transfer proxy can call system.remark",
      test: async function () {
        await context.createBlock();

        // Dave has multiple proxy types, but none of them allows to call balances.transfer
        const balanceBefore = (await polkadotJs.query.system.account(dave.address)).data.free;
        const tx = polkadotJs.tx.proxy.proxy(alice.address, null, polkadotJs.tx.system.remarkWithEvent('I was called through using proxy.proxy'));
        await context.createBlock([
          await tx.signAsync(dave),
        ]);

        const events = await polkadotJs.query.system.events();
        const ev1 = events.filter(
          (a) => {
            return a.event.method == "ProxyExecuted";
        });
        expect(ev1.length).to.be.equal(1);
        expect(ev1[0].event.data[0].toString()).to.be.eq("Ok");

        const ev2 = events.filter(
          (a) => {
            return a.event.method == "Remarked";
        });
        expect(ev2.length).to.be.equal(1);
      },
    });
    },
});
