import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type KeyringPair, alith } from "@moonwall/util";
import { type ApiPromise, Keyring } from "@polkadot/api";

import { SEPOLIA_SOVEREIGN_ACCOUNT_ADDRESS, TESTNET_ETHEREUM_NETWORK_ID } from "utils";

describeSuite({
  id: "COM0103",
  title: "XCM transfer to Ethereum",
  foundationMethods: "dev",
  testCases: ({ context, it }) => {
    let polkadotJs: ApiPromise;
    let aliceOrAlith: KeyringPair;

    beforeAll(async () => {
      polkadotJs = context.polkadotJs();
      aliceOrAlith = context.isEthereumChain ? alith : context.keyring.alice;
    });

    it({
      id: "T01",
      title: "Should allow sending asset to Ethereum",
      test: async () => {
        const ethereumNetwork = { Ethereum: { chainId: TESTNET_ETHEREUM_NETWORK_ID } };
        // Random ETH destination that we send asset to
        const destinationAddress = "0x1234567890abcdef1234567890abcdef12345678";
        const tokenToTransfer = 123_321_000_000_000_000n;

        // Check balance before transfer
        const balanceBefore = (
          await polkadotJs.query.system.account(SEPOLIA_SOVEREIGN_ACCOUNT_ADDRESS)
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

        const metadata = await polkadotJs.rpc.state.getMetadata();
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

        const tx = polkadotJs.tx.polkadotXcm.transferAssets(
          dest,
          versionedBeneficiary,
          versionedAssets,
          0,
          "Unlimited"
        );

        await tx.signAndSend(aliceOrAlith);

        await context.createBlock();

        const balanceAfter = (
          await polkadotJs.query.system.account(SEPOLIA_SOVEREIGN_ACCOUNT_ADDRESS)
        ).data.free.toBigInt();

        expect(balanceAfter - balanceBefore).toEqual(tokenToTransfer);
      },
    });
  },
});
