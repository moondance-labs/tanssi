import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { KeyringPair, extractFee } from "@moonwall/util";
import { ApiPromise, Keyring } from "@polkadot/api";
import { jumpSessions } from "../../../util/block";
import { u8aToHex, stringToHex } from "@polkadot/util";

describeSuite({
  id: "D14",
  title: "Fee test suite",
  foundationMethods: "dev",
  testCases: ({ it, context, log }) => {
    let polkadotJs: ApiPromise;
    let alice: KeyringPair;
    let bob: KeyringPair;
    let charlie: KeyringPair;
    let dave: KeyringPair;

    beforeAll(() => {
      const keyring = new Keyring({ type: "sr25519" });
      alice = keyring.addFromUri("//Alice", { name: "Alice default" });
      bob = keyring.addFromUri("//Bob", { name: "Bob default" });
      charlie = keyring.addFromUri("//Charlie", { name: "Charlie default" });
      dave = keyring.addFromUri("//Dave", { name: "Dave default" });
      polkadotJs = context.polkadotJs();
    });

    it({
        id: "E01",
        title: "Fee of balances.transfer matches expected",
        test: async function () {
        // Dave has multiple proxy types, but none of them allows to call balances.transfer
        const balanceBefore = (await polkadotJs.query.system.account(alice.address)).data.free.toBigInt();
        const tx = polkadotJs.tx.balances.transfer(bob.address, 200_000);

        // Estimate the fees as RuntimeDispatchInfo using the signer
        const info = await tx.paymentInfo(alice.address);
        console.log("INFO: ", info.toJSON());
        console.log("tx len: ", tx.encodedLength);
        const signedTx = await tx.signAsync(alice);
        console.log("signed tx len: ", signedTx.encodedLength);

        await context.createBlock([
          signedTx
        ]);

        const events = await polkadotJs.query.system.events();
        const fee = extractFee(events).amount.toBigInt();
        const expectedFee = 2630822n;

        expect(fee).to.equal(expectedFee);

        const balanceAfter = (await polkadotJs.query.system.account(alice.address)).data.free.toBigInt();

        // Balance must be old balance minus fee minus transfered value
        expect(balanceBefore - fee - 200_000n).to.equal(balanceAfter);
        },
    });

    it({
      id: "E02",
      title: "Fee of balances.transfer can be estimated in advance",
      test: async function () {
      // Dave has multiple proxy types, but none of them allows to call balances.transfer
      const balanceBefore = (await polkadotJs.query.system.account(alice.address)).data.free.toBigInt();
      const tx = polkadotJs.tx.balances.transfer(bob.address, 200_000);

      // Estimate the fees as RuntimeDispatchInfo using the signer
      const info = await tx.paymentInfo(alice.address);
      
      console.log("INFO: ", info.toJSON());
      console.log("tx len: ", tx.encodedLength);
      const signedTx = await tx.signAsync(alice);
      console.log("signed tx len: ", signedTx.encodedLength);

      await context.createBlock([
        signedTx
      ]);

      const events = await polkadotJs.query.system.events();
      const fee = extractFee(events).amount.toBigInt();
      const expectedFee = 1000144n;

      expect(fee).to.equal(expectedFee);

      const balanceAfter = (await polkadotJs.query.system.account(alice.address)).data.free.toBigInt();

      // Balance must be old balance minus fee minus transfered value
      expect(balanceBefore - fee - 200_000n).to.equal(balanceAfter);
      },
    });

    it({
      id: "E03",
      title: "Fee of balances.transfer can be estimated in advance 2",
      test: async function () {
      // Dave has multiple proxy types, but none of them allows to call balances.transfer
      const balanceBefore = (await polkadotJs.query.system.account(alice.address)).data.free.toBigInt();
      const tx = polkadotJs.tx.balances.transfer(bob.address, 200_000);

      // Estimate the fees as RuntimeDispatchInfo using the signer
      const info = await tx.paymentInfo(alice.address);
      
      console.log("INFO: ", info.toJSON());
      console.log("tx len: ", tx.encodedLength);
      const signedTx = await tx.signAsync(alice);
      console.log("signed tx len: ", signedTx.encodedLength);

      await context.createBlock([
        signedTx
      ]);

      const events = await polkadotJs.query.system.events();
      const fee = extractFee(events).amount.toBigInt();
      const expectedFee = 1000144n;

      expect(fee).to.equal(expectedFee);

      const balanceAfter = (await polkadotJs.query.system.account(alice.address)).data.free.toBigInt();

      // Balance must be old balance minus fee minus transfered value
      expect(balanceBefore - fee - 200_000n).to.equal(balanceAfter);
      },
    });
  },
});