import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { KeyringPair, extractFee } from "@moonwall/util";
import { ApiPromise, Keyring } from "@polkadot/api";

describeSuite({
  id: "DT0401",
  title: "Fee test suite",
  foundationMethods: "dev",
  testCases: ({ it, context, log }) => {
    let polkadotJs: ApiPromise;
    let alice: KeyringPair;
    let bob: KeyringPair;
    let charlie: KeyringPair;
    let dave: KeyringPair;

    beforeAll(async () => {
      alice = context.keyring.alice;
      bob = context.keyring.bob;
      charlie = context.keyring.charlie;
      dave = context.keyring.dave;
      polkadotJs = context.polkadotJs();

      // We must create an empty block before any tests, otherwise the fee of the first test will be
      // 2630822 despite the weight being the same:
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
        const expectedFee = 1000000n + BigInt(signedTx.encodedLength);
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
        const expectedFee = 1000000n + BigInt(signedTx.encodedLength);
        expect(fee).to.equal(expectedFee);

        const inclusionFee = feeDetails.inclusionFee.unwrapOrDefault();
        const tip = 0n;
        expect(fee).to.equal(inclusionFee.baseFee.toBigInt() + inclusionFee.lenFee.toBigInt() + inclusionFee.adjustedWeightFee.toBigInt() + tip);

        const balanceAfter = (await polkadotJs.query.system.account(alice.address)).data.free.toBigInt();

        // Balance must be old balance minus fee minus transfered value
        expect(balanceBefore - fee - 200_000n).to.equal(balanceAfter);
      },
    });

    it({
      id: "E03",
      title: "Fee of balances.transfer does not increase after 100 full blocks",
      test: async function () {
        let fillAmount = 600_000_000; // equal to 60% Perbill

        for (let i = 0; i < 100; i++) {
          const tx = polkadotJs.tx.rootTesting.fillBlock(fillAmount);
          const signedTx = await polkadotJs.tx.sudo.sudo(tx).signAsync(alice);
  
          await context.createBlock([
            signedTx
          ]);

          // Because the session duration is only 5 blocks, 1 out of every 5 blocks
          // cannot include any extrinsics. So we check that case, and create an
          // additional block.
          const block = await polkadotJs.rpc.chain.getBlock();
          const includedTxHashes = block.block.extrinsics.map((x) => x.hash.toString());

          if (!includedTxHashes.includes(signedTx.hash.toString())) {
            await context.createBlock([]);  
          }
        }

        const balanceBefore = (await polkadotJs.query.system.account(alice.address)).data.free.toBigInt();
        const tx = polkadotJs.tx.balances.transfer(bob.address, 200_000);
        const signedTx = await tx.signAsync(alice);
        const feeDetails = await polkadotJs.call.transactionPaymentApi.queryFeeDetails(tx, signedTx.encodedLength);
        await context.createBlock([
          signedTx
        ]);

        const events = await polkadotJs.query.system.events();
        const fee = extractFee(events).amount.toBigInt();
        const expectedFee = 1000000n + BigInt(signedTx.encodedLength);
        expect(fee).to.equal(expectedFee);

        const inclusionFee = feeDetails.inclusionFee.unwrapOrDefault();
        const tip = 0n;
        expect(fee).to.equal(inclusionFee.baseFee.toBigInt() + inclusionFee.lenFee.toBigInt() + inclusionFee.adjustedWeightFee.toBigInt() + tip);

        const balanceAfter = (await polkadotJs.query.system.account(alice.address)).data.free.toBigInt();

        // Balance must be old balance minus fee minus transfered value
        expect(balanceBefore - fee - 200_000n).to.equal(balanceAfter);
      },
    });

    it({
      id: "E04",
      title: "Fees are burned",
      test: async function () {
        await context.createBlock([]);
        const totalSupplyBefore = (await polkadotJs.query.balances.totalIssuance()).toBigInt();
        const balanceBefore = (await polkadotJs.query.system.account(alice.address)).data.free.toBigInt();
        const tx = polkadotJs.tx.balances.transfer(bob.address, 200_000);
        const signedTx = await tx.signAsync(alice);

        await context.createBlock([
          signedTx
        ]);

        const events = await polkadotJs.query.system.events();
        const fee = extractFee(events).amount.toBigInt();
        const expectedFee = 1000000n + BigInt(signedTx.encodedLength);
        expect(fee).to.equal(expectedFee);

        const balanceAfter = (await polkadotJs.query.system.account(alice.address)).data.free.toBigInt();

        // Balance must be old balance minus fee minus transfered value
        expect(balanceBefore - fee - 200_000n).to.equal(balanceAfter);

        const totalSupplyAfter = (await polkadotJs.query.balances.totalIssuance()).toBigInt();

        expect(totalSupplyBefore - totalSupplyAfter).to.equal(fee);
      },
    });
  },
});