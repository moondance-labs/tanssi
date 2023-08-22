import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { KeyringPair, extractFee } from "@moonwall/util";
import { ApiPromise, Keyring } from "@polkadot/api";

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

    beforeAll(async () => {
      const keyring = new Keyring({ type: "sr25519" });
      alice = keyring.addFromUri("//Alice", { name: "Alice default" });
      bob = keyring.addFromUri("//Bob", { name: "Bob default" });
      charlie = keyring.addFromUri("//Charlie", { name: "Charlie default" });
      dave = keyring.addFromUri("//Dave", { name: "Dave default" });
      polkadotJs = context.polkadotJs();

      // We must create an empty block before any tests, otherwise the fee is 2630822
      // despite the weight being the same:
      //   inclusionFee: { baseFee: 1000000, lenFee: 144, adjustedWeightFee: 1630678 }
      await context.createBlock([]);
    });

    it({
      id: "E01",
      title: "Fee of balances.transfer can be estimated using paymentInfo",
      test: async function () {
      const balanceBefore = (await polkadotJs.query.system.account(alice.address)).data.free.toBigInt();
      const tx = polkadotJs.tx.balances.transfer(bob.address, 200_000);
      const info = await tx.paymentInfo(alice.address);
      const signedTx = await tx.signAsync(alice);
      await context.createBlock([
        signedTx
      ]);

      const events = await polkadotJs.query.system.events();
      const fee = extractFee(events).amount.toBigInt();
      const expectedFee = 1000144n;
      expect(fee).to.equal(expectedFee);

      const tip = 0n;
      expect(fee).to.equal(info.partialFee.toBigInt() + tip);

      const balanceAfter = (await polkadotJs.query.system.account(alice.address)).data.free.toBigInt();
      // Balance must be old balance minus fee minus transfered value
      expect(balanceBefore - fee - 200_000n).to.equal(balanceAfter);
      },
    });

    it({
      id: "E02",
      title: "Fee of balances.transfer can be estimated using transactionPaymentApi.queryFeeDetails",
      test: async function () {
      const balanceBefore = (await polkadotJs.query.system.account(alice.address)).data.free.toBigInt();
      const tx = polkadotJs.tx.balances.transfer(bob.address, 200_000);
      const signedTx = await tx.signAsync(alice);
      const feeDetails = await polkadotJs.call.transactionPaymentApi.queryFeeDetails(tx, signedTx.encodedLength);

      await context.createBlock([
        signedTx
      ]);

      const events = await polkadotJs.query.system.events();
      const fee = extractFee(events).amount.toBigInt();
      const expectedFee = 1000144n;
      expect(fee).to.equal(expectedFee);

      const inclusionFee = feeDetails.inclusionFee.unwrapOrDefault();
      const tip = 0n;
      expect(fee).to.equal(inclusionFee.baseFee.toBigInt() + inclusionFee.lenFee.toBigInt() + inclusionFee.adjustedWeightFee.toBigInt() + tip);

      const balanceAfter = (await polkadotJs.query.system.account(alice.address)).data.free.toBigInt();

      // Balance must be old balance minus fee minus transfered value
      expect(balanceBefore - fee - 200_000n).to.equal(balanceAfter);
      },
  });
  },
});