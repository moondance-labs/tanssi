// @ts-nocheck

import "@tanssi/api-augment";
import { beforeAll, describeSuite, expect, isExtrinsicSuccessful } from "@moonwall/cli";
import type { ApiPromise } from "@polkadot/api";
import type { KeyringPair } from "@moonwall/util";
import { retrieveSudoDispatchErrors } from "helpers";

describeSuite({
    id: "DEVTPRORAC",
    title: "Pallet services payment price oracle test suite",
    foundationMethods: "dev",

    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let sudo_alice: KeyringPair;
        let bob: KeyringPair;
        let proxy_account: KeyringPair;
        let isStarlight = false;
        let chain: string;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            sudo_alice = context.keyring.alice;
            bob = context.keyring.bob;
            proxy_account = context.keyring.charlie;
            chain = polkadotJs.consts.system.version.specName.toString();
            isStarlight = chain === "starlight";
        });

        it({
            id: "E01",
            title: "When price is not set, it uses default FIXED cost (dancelight and starlight)",
            test: async () => {
                const currentPrice = await polkadotJs.query.servicesPaymentPriceOracle.tokenPriceUsd();
                expect(currentPrice.isNone).to.be.true; // Ensure price is not set

                let expectedFixedBlockCost = 1_000_000n;
                let expectedFixedCollatorAssignmentCost = 100_000_000n;

                if (isStarlight) {
                    expectedFixedBlockCost = 30_000_000_000n;
                    expectedFixedCollatorAssignmentCost = 50_000_000_000_000n;
                }

                const apiBlockCost = await polkadotJs.call.servicesPaymentPriceOracleApi.blockCost();
                const apiCollatorAssignmentCost =
                    await polkadotJs.call.servicesPaymentPriceOracleApi.collatorAssignmentCost();

                expect(apiBlockCost.toBigInt()).to.equal(expectedFixedBlockCost);
                expect(apiCollatorAssignmentCost.toBigInt()).to.equal(expectedFixedCollatorAssignmentCost);
            },
        });

        it({
            id: "E02",
            title: "Proxy with type SudoOraclePrice can update price",
            test: async () => {
                const testPrice = polkadotJs.createType("FixedU128", "1234567890123456789");
                const proxyType = "SudoOraclePrice";

                // Add proxy
                const addProxyTx = polkadotJs.tx.proxy.addProxy(proxy_account.address, proxyType, 0);
                await context.createBlock([await addProxyTx.signAsync(sudo_alice)]);
                let events = await polkadotJs.query.system.events();
                expect(isExtrinsicSuccessful(events)).to.be.true;

                // Check that the proxy has been added
                const proxies = await polkadotJs.query.proxy.proxies(sudo_alice.address);
                expect(proxies[0].toHuman()[0]).to.deep.include({
                    delegate: proxy_account.address,
                    proxyType: proxyType,
                });

                // Set token price via proxy
                const setPriceViaProxyTx = polkadotJs.tx.proxy.proxy(
                    sudo_alice.address,
                    proxyType,
                    polkadotJs.tx.sudo.sudo(polkadotJs.tx.servicesPaymentPriceOracle.setTokenPrice(testPrice))
                );
                await context.createBlock([await setPriceViaProxyTx.signAsync(proxy_account)]);
                events = await polkadotJs.query.system.events();
                expect(isExtrinsicSuccessful(events)).to.be.true;

                // Verify price update
                const currentPrice = await polkadotJs.query.servicesPaymentPriceOracle.tokenPriceUsd();
                expect(currentPrice.unwrap().toBigInt().toString()).to.equal(testPrice.toBigInt().toString());
            },
        });

        it({
            id: "E03",
            title: "Only sudo can update price",
            test: async () => {
                const testPrice = polkadotJs.createType("FixedU128", "1234567890123456789");

                const setPriceTx = polkadotJs.tx.servicesPaymentPriceOracle.setTokenPrice(testPrice);
                const {
                    result: [setPriceAttempt],
                } = await context.createBlock([await setPriceTx.signAsync(bob)]);
                const events = await polkadotJs.query.system.events();

                const ev1 = events.filter((a) => {
                    return a.event.method === "ExtrinsicFailed";
                });
                expect(ev1.length).to.be.equal(1);

                expect(setPriceAttempt.successful).toEqual(false);
                expect(setPriceAttempt.error.name).toEqual("BadOrigin");
            },
        });

        it({
            id: "E04",
            title: "Set token price at bounds succeeded",
            test: async () => {
                const minPriceConst = polkadotJs.consts.servicesPaymentPriceOracle.minTokenPrice.toBigInt();
                const maxPriceConst = polkadotJs.consts.servicesPaymentPriceOracle.maxTokenPrice.toBigInt();

                const minPrice = polkadotJs.createType("FixedU128", minPriceConst);
                const maxPrice = polkadotJs.createType("FixedU128", maxPriceConst);

                // Test with MinTokenPrice
                const setMinPriceTx = polkadotJs.tx.sudo.sudo(
                    polkadotJs.tx.servicesPaymentPriceOracle.setTokenPrice(minPrice)
                );
                await context.createBlock([await setMinPriceTx.signAsync(sudo_alice)]);
                let events = await polkadotJs.query.system.events();
                expect(isExtrinsicSuccessful(events)).to.be.true;
                let currentPrice = await polkadotJs.query.servicesPaymentPriceOracle.tokenPriceUsd();
                expect(currentPrice.unwrap().toBigInt().toString()).to.equal(minPrice.toBigInt().toString());

                // Test with MaxTokenPrice
                const setMaxPriceTx = polkadotJs.tx.sudo.sudo(
                    polkadotJs.tx.servicesPaymentPriceOracle.setTokenPrice(maxPrice)
                );
                await context.createBlock([await setMaxPriceTx.signAsync(sudo_alice)]);
                events = await polkadotJs.query.system.events();
                expect(isExtrinsicSuccessful(events)).to.be.true;
                currentPrice = await polkadotJs.query.servicesPaymentPriceOracle.tokenPriceUsd();
                expect(currentPrice.unwrap().toBigInt().toString()).to.equal(maxPrice.toBigInt().toString());
            },
        });

        it({
            id: "E05",
            title: "Set token price outside bounds FAILED",
            test: async () => {
                const minPriceConst = polkadotJs.consts.servicesPaymentPriceOracle.minTokenPrice.toBigInt();
                const maxPriceConst = polkadotJs.consts.servicesPaymentPriceOracle.maxTokenPrice.toBigInt();

                // Price below minimum
                // Ensure belowMinPriceValue is not zero, as price cannot be zero.
                const belowMinPriceValue = minPriceConst > 1n ? minPriceConst - 1n : minPriceConst;
                const belowMinPrice = polkadotJs.createType("FixedU128", belowMinPriceValue);

                const setBelowMinPriceTx = polkadotJs.tx.sudo.sudo(
                    polkadotJs.tx.servicesPaymentPriceOracle.setTokenPrice(belowMinPrice)
                );
                await context.createBlock([await setBelowMinPriceTx.signAsync(sudo_alice)]);
                const errorEvents = await retrieveSudoDispatchErrors(context.polkadotJs());
                expect(errorEvents.length, "Amount of error events should be 1").toBe(1);
                expect(errorEvents[0].method).to.be.eq("PriceOutOfBounds");

                // Price above maximum
                const aboveMaxPriceValue = maxPriceConst + 1n;
                const aboveMaxPrice = polkadotJs.createType("FixedU128", aboveMaxPriceValue);

                const setAboveMaxPriceTx = polkadotJs.tx.sudo.sudo(
                    polkadotJs.tx.servicesPaymentPriceOracle.setTokenPrice(aboveMaxPrice)
                );
                await context.createBlock([await setAboveMaxPriceTx.signAsync(sudo_alice)]);

                const errorEvents2 = await retrieveSudoDispatchErrors(context.polkadotJs());
                expect(errorEvents2.length, "Amount of error events should be 1").toBe(1);
                expect(errorEvents2[0].method).to.be.eq("PriceOutOfBounds");
            },
        });

        it({
            id: "E06",
            title: "Runtime API returns correct cost for block production and collator assignment when price is set",
            test: async () => {
                // Set a known price
                const setPriceValue = 1_000_000_000_000_000_000n; // $1
                const setPriceFixedU128 = polkadotJs.createType("FixedU128", setPriceValue);
                const setPriceTx = polkadotJs.tx.sudo.sudo(
                    polkadotJs.tx.servicesPaymentPriceOracle.setTokenPrice(setPriceFixedU128)
                );
                await context.createBlock([await setPriceTx.signAsync(sudo_alice)]);
                const events = await polkadotJs.query.system.events();
                expect(isExtrinsicSuccessful(events)).to.be.true;

                const currentPrice = await polkadotJs.query.servicesPaymentPriceOracle.tokenPriceUsd();
                expect(currentPrice.unwrap().toBigInt()).to.equal(setPriceFixedU128.toBigInt());

                // Check that costs has changed:
                const apiBlockCost1 = await polkadotJs.call.servicesPaymentPriceOracleApi.blockCost();
                const apiCollatorAssignmentCost1 =
                    await polkadotJs.call.servicesPaymentPriceOracleApi.collatorAssignmentCost();

                let apiBlockCost = "420875420";
                let apiCollatorAssignmentCost = "42087542087";

                if (isStarlight) {
                    apiBlockCost = "27612105";
                    apiCollatorAssignmentCost = "46020175244";
                }

                expect(apiBlockCost1.toString()).to.equal(apiBlockCost);
                expect(apiCollatorAssignmentCost1.toString()).to.equal(apiCollatorAssignmentCost);
            },
        });
    },
});
