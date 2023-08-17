import { beforeAll, describeSuite, expect, customDevRpcRequest } from "@moonwall/cli";
import { ApiPromise } from "@polkadot/api";
import {
  XcmFragment,
  injectDmpMessageAndSeal,
  descendParentOriginFromAddress32,
} from "../../../util/xcm.js";
import { generateKeyringPair } from "@moonwall/util";
import { expectOk } from "../../../util/expect.ts";
import { Keyring } from "@polkadot/api";
import { BN } from "@polkadot/util";

describeSuite({
  id: "D1",
  title: "Mock XCM - Succeeds using sovereign accounts",
  foundationMethods: "dev",
  testCases: ({ context, it, log }) => {
    let polkadotJs: ApiPromise;
    let transferredBalance;
    let sendingAddress;
    let descendAddress;
    let alice;


    beforeAll(async function () {
        const keyring = new Keyring({ type: 'sr25519' });
        alice = keyring.addFromUri('//Alice', { name: 'Alice default' });

        const { originAddress, descendOriginAddress } = descendParentOriginFromAddress32(context);

        sendingAddress = originAddress;
        descendAddress = descendOriginAddress;

        transferredBalance = 10_000_000_000_000n;
        polkadotJs = context.polkadotJs();

        const txSigned = polkadotJs.tx.balances.transfer(descendOriginAddress, transferredBalance);

        await expectOk(
          context.createBlock(
            await txSigned.signAsync(alice)
          )
        );
        const balanceSigned = (
          (await polkadotJs.query.system.account(descendOriginAddress)) as any
        ).data.free.toBigInt();
        expect(balanceSigned).to.eq(transferredBalance);
    });

    it({
      id: "T01",
      title: "Should succeed using sovereign account from signed origin",
      test: async function () {
        // Generate random receiver address
        let random: KeyringPair;
        random = generateKeyringPair("sr25519");

        // Get Pallet balances index
        const metadata = await polkadotJs.rpc.state.getMetadata();
        const balancesPalletIndex = (metadata.asLatest.toHuman().pallets as Array<any>).find(
        (pallet) => {
            return pallet.name === "Balances";
        }
        ).index;

        const transferCall = polkadotJs.tx.balances.transfer(
        random.address,
        transferredBalance / 10n
        );
        const transferCallEncoded = transferCall?.method.toHex();

        // We are going to test that we can receive a transact operation from parachain 1
        // using descendOrigin first
        const xcmMessage = new XcmFragment({
            assets: [
                {
                multilocation: {
                    parents: 0,
                    interior: {
                    X1: { PalletInstance: balancesPalletIndex },
                    },
                },
                fungible: transferredBalance / 4n,
                },
            ],
            descend_origin: sendingAddress,
        })
        .descend_origin()
        .withdraw_asset()
        .buy_execution()
        .push_any({
            Transact: {
            originType: "SovereignAccount",
            requireWeightAtMost: new BN(1000000000),
            call: {
                encoded: transferCallEncoded,
            },
            },
        })
        .as_v2();

        // Send an XCM and create block to execute it
        await injectDmpMessageAndSeal(context, {
            type: "XcmVersionedXcm",
            payload: xcmMessage,
        } as RawXcmMessage);

        let events = polkadotJs.query.system.events();

        // Make sure the state has ALITH's foreign parachain tokens
        const testAccountBalance = (
        await polkadotJs.query.system.account(random.address)
        ).data.free.toBigInt();

        expect(testAccountBalance).to.eq(transferredBalance / 10n);
        },
    });
  },
});