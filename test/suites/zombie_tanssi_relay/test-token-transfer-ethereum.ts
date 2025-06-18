import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type KeyringPair, alith } from "@moonwall/util";
import { type ApiPromise, Keyring } from "@polkadot/api";

import {
  SEPOLIA_SOVEREIGN_ACCOUNT_ADDRESS,
  signAndSendAndInclude,
  TESTNET_ETHEREUM_NETWORK_ID,
} from "utils";

describeSuite({
  id: "ZOMBIETANSS02",
  title: "XCM transfer to Ethereum",
  foundationMethods: "zombie",
  testCases: ({ context, it }) => {
    let containerChainPolkadotJs: ApiPromise;
    let alice: KeyringPair;
    let aliceAccount32: KeyringPair;
    let chain: string;

    beforeAll(async () => {
      containerChainPolkadotJs = context.polkadotJs("Container2001");
      chain = containerChainPolkadotJs.consts.system.version.specName.toString();
      aliceAccount32 = new Keyring({ type: "sr25519" }).addFromUri("//Alice", {
        name: "Alice default",
      });
      alice = chain === "frontier-template" ? alith : aliceAccount32;
    });

    it({
      id: "T01",
      title: "Should allow sending asset to Ethereum",
      test: async () => {
        const ethereumNetwork = { Ethereum: { chainId: TESTNET_ETHEREUM_NETWORK_ID } };
        // Random ETH destination that we send asset to
        const destinationAddress = "0x1234567890abcdef1234567890abcdef12345678";
        const holdingAccount = SEPOLIA_SOVEREIGN_ACCOUNT_ADDRESS;
        const tokenToTransfer = 123_321_000_000_000_000n;

        // Check balance before transfer
        const balanceBefore = (
          await containerChainPolkadotJs.query.system.account(holdingAccount)
        ).data.free.toBigInt();

        const versionedBeneficiary = {
          V3: {
            parents: 0,
            interior: {
              X1: {
                AccountKey20: {
                  network: ethereumNetwork,
                  key: destinationAddress,
                },
              },
            },
          },
        };

        const metadata = await containerChainPolkadotJs.rpc.state.getMetadata();
        const balancesPalletIndex = metadata.asLatest.pallets
          .find(({ name }) => name.toString() === "Balances")
          .index.toNumber();

        const assetToTransferNative = {
          id: {
            Concrete: {
              parents: 0,
              interior: {
                X1: { PalletInstance: Number(balancesPalletIndex) },
              },
            },
          },
          fun: { Fungible: tokenToTransfer },
        };

        const versionedAssets = {
          V3: [assetToTransferNative],
        };

        // Specify ethereum destination with global consensus
        const dest = {
          V3: {
            parents: 2,
            interior: {
              X1: {
                GlobalConsensus: ethereumNetwork,
              },
            },
          },
        };

        const txRoot = containerChainPolkadotJs.tx.polkadotXcm.transferAssets(
          dest,
          versionedBeneficiary,
          versionedAssets,
          0,
          "Unlimited"
        );

        await signAndSendAndInclude(txRoot, alice);

        const balanceAfter = (
          await containerChainPolkadotJs.query.system.account(holdingAccount)
        ).data.free.toBigInt();

        expect(balanceAfter - balanceBefore).toEqual(tokenToTransfer);
      },
    });
  },
});
