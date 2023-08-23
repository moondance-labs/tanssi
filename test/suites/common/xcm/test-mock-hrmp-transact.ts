import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { ApiPromise } from "@polkadot/api";
import {
  XcmFragment,
  injectHrmpMessageAndSeal,
  sovereignAccountOfSiblingForAddress32,
  sovereignAccountOfSiblingForAddress20,
  descendSiblingOriginFromAddress32,
  descendSiblingOriginFromAddress20
} from "../../../util/xcm.ts";
import { generateKeyringPair } from "@moonwall/util";
import { Keyring } from "@polkadot/api";
import { BN } from "@polkadot/util";
import { alith } from "@moonwall/util";

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
    let chain;


    beforeAll(async function () {
        polkadotJs = context.polkadotJs();
        chain = polkadotJs.consts.system.version.specName.toString();
        alice = chain == 'frontier-template' ? alith : (new Keyring({ type: 'sr25519' }).addFromUri('//Alice', { name: 'Alice default' }));
        let aliceNonce = (await polkadotJs.query.system.account(alice.address)).nonce;

        let descendFunction = chain == 'frontier-template' ? descendSiblingOriginFromAddress20 : descendSiblingOriginFromAddress32;
        let sovereignFunction = chain == 'frontier-template' ? sovereignAccountOfSiblingForAddress20 : sovereignAccountOfSiblingForAddress32;

        const { originAddress, descendOriginAddress } = descendFunction(context);
        const sovereign = sovereignFunction(context, 1);

        sendingAddress = originAddress;
        descendAddress = descendOriginAddress;

        transferredBalance = 10_000_000_000_000n;
        polkadotJs = context.polkadotJs();

        const txSigned = polkadotJs.tx.balances.transfer(descendOriginAddress, transferredBalance);
        const txRoot = polkadotJs.tx.balances.transfer(sovereign, transferredBalance);

        await context.createBlock(
            await txSigned.signAsync(alice, { nonce: aliceNonce++ }),
            { allowFailures: false }
        )
        await context.createBlock(
            await txRoot.signAsync(alice, { nonce: aliceNonce++ }),
            { allowFailures: false }
        )
        const balanceSigned = 
          (await polkadotJs.query.system.account(descendOriginAddress))
        .data.free.toBigInt();
        expect(balanceSigned).to.eq(transferredBalance);
        const balanceRoot = 
            (await polkadotJs.query.system.account(sovereign)).data.free.toBigInt();
        expect(balanceRoot).to.eq(transferredBalance);
    });

    it({
      id: "T01",
      title: "Should succeed using sovereign account from signed origin",
      test: async function () {
        // Generate random receiver address
        let random: KeyringPair;
        random = chain == 'frontier-template' ? generateKeyringPair() : generateKeyringPair("sr25519");

        // Get Pallet balances index
        const metadata = await polkadotJs.rpc.state.getMetadata();
        const balancesPalletIndex = metadata.asLatest.pallets
            .find(({ name }) => name.toString() == "Balances")!
            .index.toNumber();

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
            originKind: "SovereignAccount",
            requireWeightAtMost: {
                refTime: 1000000000,
                proofSize: 32000,
            },
            call: {
                encoded: transferCallEncoded,
            },
            },
        })
        .as_v3();

        // Send an XCM and create block to execute it
        await injectHrmpMessageAndSeal(context, 1, {
            type: "XcmVersionedXcm",
            payload: xcmMessage,
        } as RawXcmMessage);

        // Make sure the state has ALITH's foreign parachain tokens
        const testAccountBalance = (
        await polkadotJs.query.system.account(random.address)
        ).data.free.toBigInt();

        expect(testAccountBalance).to.eq(transferredBalance / 10n);
        },
    });

    it({
        id: "T02",
        title: "Should succeed using sovereign account from root origin",
        test: async function () {
            // Generate random receiver address
            let random: KeyringPair;
            random = chain == 'frontier-template' ? generateKeyringPair() : generateKeyringPair("sr25519");

            // Get Pallet balances index
            const metadata = await polkadotJs.rpc.state.getMetadata();
            const balancesPalletIndex = metadata.asLatest.pallets
                .find(({ name }) => name.toString() == "Balances")!
                .index.toNumber();
            
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
                ]
            })
            .withdraw_asset()
            .buy_execution()
            .push_any({
                Transact: {
                originKind: "SovereignAccount",
                requireWeightAtMost: {
                    refTime: 1000000000,
                    proofSize: 32000,
                },
                call: {
                    encoded: transferCallEncoded,
                },
                },
            })
            .as_v3();
    
    
            // Send an XCM and create block to execute it
            await injectHrmpMessageAndSeal(context, 1, {
                type: "XcmVersionedXcm",
                payload: xcmMessage,
            } as RawXcmMessage);
    
            // Make sure the state has ALITH's foreign parachain tokens
            const testAccountBalance = (
            await polkadotJs.query.system.account(random.address)
            ).data.free.toBigInt();
    
            expect(testAccountBalance).to.eq(transferredBalance / 10n);
            },
      });
  },
});