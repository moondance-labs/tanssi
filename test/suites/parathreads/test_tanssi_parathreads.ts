import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { MIN_GAS_PRICE, customWeb3Request, generateKeyringPair, getBlockArray } from "@moonwall/util";
import { ApiPromise, Keyring } from "@polkadot/api";
import { Signer } from "ethers";
import fs from "fs/promises";
import { getAuthorFromDigest } from "../../util/author";
import { signAndSendAndInclude, waitSessions } from "../../util/block";
import { createTransfer, waitUntilEthTxIncluded } from "../../util/ethereum";
import { getKeyringNimbusIdHex } from "../../util/keys";
import { getHeaderFromRelay } from "../../util/relayInterface";
import { chainSpecToContainerChainGenesisData } from "../../util/genesis_data.ts";
import jsonBg from "json-bigint";
import Bottleneck from "bottleneck";
import { stringToHex } from "@polkadot/util";
const JSONbig = jsonBg({ useNativeBigInt: true });

describeSuite({
    id: "R01",
    title: "Zombie Tanssi Rotation Test",
    foundationMethods: "zombie",
    testCases: function ({ it, context }) {
        let paraApi: ApiPromise;
        let relayApi: ApiPromise;
        let container2000Api: ApiPromise;
        let container2001Api: ApiPromise;
        let ethersSigner: Signer;
        let allCollators: string[];
        let collatorName: Record<string, string>;

        beforeAll(async () => {
            paraApi = context.polkadotJs("Tanssi");
            relayApi = context.polkadotJs("Relay");
            container2000Api = context.polkadotJs("Container2000");
            container2001Api = context.polkadotJs("Container2001");
            ethersSigner = context.ethers();

            const relayNetwork = relayApi.consts.system.version.specName.toString();
            expect(relayNetwork, "Relay API incorrect").to.contain("rococo");

            const paraNetwork = paraApi.consts.system.version.specName.toString();
            const paraId1000 = (await paraApi.query.parachainInfo.parachainId()).toString();
            expect(paraNetwork, "Para API incorrect").to.contain("dancebox");
            expect(paraId1000, "Para API incorrect").to.be.equal("1000");

            const container2000Network = container2000Api.consts.system.version.specName.toString();
            const paraId2000 = (await container2000Api.query.parachainInfo.parachainId()).toString();
            expect(container2000Network, "Container2000 API incorrect").to.contain("container-chain-template");
            expect(paraId2000, "Container2000 API incorrect").to.be.equal("2000");

            const container2001Network = container2001Api.consts.system.version.specName.toString();
            const paraId2001 = (await container2001Api.query.parachainInfo.parachainId()).toString();
            expect(container2001Network, "Container2001 API incorrect").to.contain("frontier-template");
            expect(paraId2001, "Container2001 API incorrect").to.be.equal("2001");

            // Test block numbers in relay are 0 yet
            const header2000 = await getHeaderFromRelay(relayApi, 2000);
            const header2001 = await getHeaderFromRelay(relayApi, 2001);

            expect(header2000.number.toNumber()).to.be.equal(0);
            expect(header2001.number.toNumber()).to.be.equal(0);

            // Initialize list of all collators, this should match the names from build-spec.sh script
            allCollators = ["Collator-01", "Collator-02", "Collator-03", "Collator-04"];
            // Initialize reverse map of collator key to collator name
            collatorName = createCollatorKeyToNameMap(paraApi, allCollators);
            console.log(collatorName);
        }, 120000);

        it({
            id: "T01",
            title: "Blocks are being produced on parachain",
            test: async function () {
                const blockNum = (await paraApi.rpc.chain.getBlock()).block.header.number.toNumber();
                expect(blockNum).to.be.greaterThan(0);
            },
        });

        it({
            id: "T02",
            title: "Disable full_rotation",
            timeout: 120000,
            test: async function () {
                const keyring = new Keyring({ type: "sr25519" });
                const alice = keyring.addFromUri("//Alice", { name: "Alice default" });
                const tx4 = await paraApi.tx.configuration.setFullRotationPeriod(0);
                await signAndSendAndInclude(paraApi.tx.sudo.sudo(tx4), alice);
            },
        });

        it({
            id: "T03a",
            title: "Register parathreads 2000 and 2001",
            timeout: 240000,
            test: async function () {
                const keyring = new Keyring({ type: "sr25519" });
                const alice = keyring.addFromUri("//Alice", { name: "Alice default" });
                const txs2000 = await registerParathread(paraApi, alice.address, 2000);
                const txs2001 = await registerParathread(paraApi, alice.address, 2001);

                const slotFrequency2000 = paraApi.createType("TpTraitsSlotFrequency", {
                    min: 5,
                    max: 5,
                });
                const tx1 = await paraApi.tx.registrar.setParathreadParams(2000, slotFrequency2000);
                const slotFrequency2001 = paraApi.createType("TpTraitsSlotFrequency", {
                    min: 2,
                    max: 2,
                });
                const tx2 = await paraApi.tx.registrar.setParathreadParams(2001, slotFrequency2001);
                const txs = paraApi.tx.utility.batchAll([...txs2000, ...txs2001, tx1, tx2]);
                await signAndSendAndInclude(paraApi.tx.sudo.sudo(txs), alice);
            },
        });

        it({
            id: "T03b",
            title: "Wait for parathreads 2000 and 2001 to be assigned collators",
            timeout: 600000,
            test: async function () {
                await waitSessions(context, paraApi, 2, async () => {
                    const currentSession = (await paraApi.query.session.currentIndex()).toNumber();
                    const containerChainCollators = (
                        await paraApi.query.authorityAssignment.collatorContainerChain(currentSession)
                    ).toJSON().containerChains;
                    // Stop waiting when parathreads have been assigned collators
                    return containerChainCollators[2000] != undefined && containerChainCollators[2001] != undefined;
                });
            },
        });

        it({
            id: "T04",
            title: "Blocks are being produced on container 2000",
            test: async function () {
                // Produces 1 block every 5 slots, which is every 60 seconds
                // Give it a bit more time just in case
                await sleep(120000);
                const blockNum = (await container2000Api.rpc.chain.getBlock()).block.header.number.toNumber();
                expect(blockNum).to.be.greaterThan(0);
            },
        });

        it({
            id: "T05",
            title: "Blocks are being produced on container 2001",
            test: async function () {
                // Produces 1 block every 2 slots, which is every 24 seconds
                await sleep(24000);
                const blockNum = (await container2001Api.rpc.chain.getBlock()).block.header.number.toNumber();

                expect(blockNum).to.be.greaterThan(0);
                expect(await ethersSigner.provider.getBlockNumber(), "Safe tag is not present").to.be.greaterThan(0);
            },
        });

        it({
            id: "T06",
            title: "Test container chain 2000 assignation is correct",
            test: async function () {
                const currentSession = (await paraApi.query.session.currentIndex()).toNumber();
                const paraId = (await container2000Api.query.parachainInfo.parachainId()).toString();
                const containerChainCollators = (
                    await paraApi.query.authorityAssignment.collatorContainerChain(currentSession)
                ).toJSON().containerChains[paraId];

                // TODO: fix once we have types
                const writtenCollators = (await container2000Api.query.authoritiesNoting.authorities()).toJSON();

                expect(containerChainCollators).to.deep.equal(writtenCollators);
            },
        });

        it({
            id: "T07",
            title: "Test container chain 2001 assignation is correct",
            test: async function () {
                const currentSession = (await paraApi.query.session.currentIndex()).toNumber();
                const paraId = (await container2001Api.query.parachainInfo.parachainId()).toString();
                const containerChainCollators = (
                    await paraApi.query.authorityAssignment.collatorContainerChain(currentSession)
                ).toJSON().containerChains[paraId];

                const writtenCollators = (await container2001Api.query.authoritiesNoting.authorities()).toJSON();

                expect(containerChainCollators).to.deep.equal(writtenCollators);
            },
        });

        it({
            id: "T08",
            title: "Test author noting is correct for both containers",
            timeout: 120000,
            test: async function () {
                const assignment = await paraApi.query.collatorAssignment.collatorContainerChain();
                const paraId2000 = await container2000Api.query.parachainInfo.parachainId();
                const paraId2001 = await container2001Api.query.parachainInfo.parachainId();

                // TODO: fix once we have types
                const containerChainCollators2000 = assignment.containerChains.toJSON()[paraId2000.toString()];
                const containerChainCollators2001 = assignment.containerChains.toJSON()[paraId2001.toString()];

                await context.waitBlock(3, "Tanssi");
                const author2000 = await paraApi.query.authorNoting.latestAuthor(paraId2000);
                const author2001 = await paraApi.query.authorNoting.latestAuthor(paraId2001);

                expect(containerChainCollators2000.includes(author2000.toJSON().author)).to.be.true;
                expect(containerChainCollators2001.includes(author2001.toJSON().author)).to.be.true;
            },
        });

        it({
            id: "T09",
            title: "Test author is correct in Orchestrator",
            test: async function () {
                const sessionIndex = (await paraApi.query.session.currentIndex()).toNumber();
                const authorities = await paraApi.query.authorityAssignment.collatorContainerChain(sessionIndex);
                const author = await getAuthorFromDigest(paraApi);
                // TODO: fix once we have types
                expect(authorities.toJSON().orchestratorChain.includes(author.toString())).to.be.true;
            },
        });

        it({
            id: "T10",
            title: "Test frontier template isEthereum",
            test: async function () {
                // TODO: fix once we have types
                const genesisData2000 = await paraApi.query.registrar.paraGenesisData(2000);
                expect(genesisData2000.toJSON().properties.isEthereum).to.be.false;
                const genesisData2001 = await paraApi.query.registrar.paraGenesisData(2001);
                expect(genesisData2001.toJSON().properties.isEthereum).to.be.true;
            },
        });
        it({
            id: "T11",
            title: "Transactions can be made with ethers",
            timeout: 120000,
            test: async function () {
                const randomAccount = generateKeyringPair();
                const tx = await createTransfer(context, randomAccount.address, 1_000_000_000_000, {
                    gasPrice: MIN_GAS_PRICE,
                });
                const txHash = await customWeb3Request(context.web3(), "eth_sendRawTransaction", [tx]);
                await waitUntilEthTxIncluded(
                    () => context.waitBlock(1, "Container2001"),
                    context.web3(),
                    txHash.result
                );
                expect(Number(await context.web3().eth.getBalance(randomAccount.address))).to.be.greaterThan(0);
            },
        });
        it({
            id: "T12",
            title: "Check block frequency of parathreads",
            timeout: 240000,
            test: async function () {
                // Wait 2 sessions so that parathreads have produced at least a few blocks each
                await waitSessions(context, paraApi, 2);

                // TODO: calculate block frequency somehow
                assertSlotFrequency(await getBlockData(paraApi), 1);
                assertSlotFrequency(await getBlockData(container2000Api), 5);
                assertSlotFrequency(await getBlockData(container2001Api), 2);
            },
        });
    },
});

async function getBlockData(api) {
    const timePeriod = 1 * 60 * 60 * 1000; // 1 hour
    const blockNumArray = await getBlockArray(api, timePeriod);

    const getBlockData = async (blockNum: number) => {
        const blockHash = await api.rpc.chain.getBlockHash(blockNum);
        const signedBlock = await api.rpc.chain.getBlock(blockHash);
        const apiAt = await api.at(blockHash);

        return {
            blockNum: blockNum,
            extrinsics: signedBlock.block.extrinsics,
            events: await apiAt.query.system.events(),
            logs: signedBlock.block.header.digest.logs,
        };
    };
    const limiter = new Bottleneck({ maxConcurrent: 5, minTime: 100 });
    const blockData = await Promise.all(blockNumArray.map((num) => limiter.schedule(() => getBlockData(num))));
    return blockData;
}

async function assertSlotFrequency(blockData, expectedSlotDiff) {
    const slotNumbers = blockData
        .map(({ logs }) => {
            const slotLog = logs.find(
                (log) => log.isPreRuntime === true && log.asPreRuntime[0].toHex() === stringToHex("aura")
            );
            return slotLog ? parseInt(slotLog.asPreRuntime[1].reverse().toString("hex"), 16) : null;
        })
        .filter((slot) => slot !== null); // Filter out nulls (blocks without slotLog)

    if (slotNumbers.length < 2) {
        throw new Error("Insufficient data for slot time calculation.");
    }

    // Calculate differences between consecutive slots
    const slotDiffs = [];
    for (let i = 1; i < slotNumbers.length; i++) {
        slotDiffs.push(slotNumbers[i] - slotNumbers[i - 1]);
    }

    // Calculate average slot difference
    const avgSlotDiff = slotDiffs.reduce((acc, diff) => acc + diff, 0) / slotDiffs.length;
    expect(
        Math.abs(avgSlotDiff - expectedSlotDiff),
        `Average slot time is different from expected: average ${avgSlotDiff}, expected ${expectedSlotDiff}`
    ).to.be.lessThan(0.5);
}

/// Create a map of collator key "5C5p..." to collator name "Collator1000-01".
function createCollatorKeyToNameMap(paraApi, collatorNames: string[]): Record<string, string> {
    const collatorName: Record<string, string> = {};

    collatorNames.forEach((name) => {
        const hexAddress = getKeyringNimbusIdHex(name);
        const k = paraApi.createType("AccountId", hexAddress);
        collatorName[k] = name;
    });

    return collatorName;
}

async function registerParathread(api, manager, paraId) {
    const specPaths = {
        2000: "specs/parathreads-template-container-2000.json",
        2001: "specs/parathreads-template-container-2001.json",
    };
    if (!specPaths[paraId]) {
        throw new Error(`Unknown chain spec path for paraId ${paraId}`);
    }
    const chain = specPaths[paraId];
    const parathread = true;
    const rawSpec = JSONbig.parse(await fs.readFile(chain, "utf8"));

    const containerChainGenesisData = chainSpecToContainerChainGenesisData(api, rawSpec);
    const txs = [];
    let tx1;
    if (parathread) {
        const slotFreq = api.createType("TpTraitsSlotFrequency", {
            min: 1,
            max: 1,
        });
        tx1 = api.tx.registrar.registerParathread(rawSpec.para_id, slotFreq, containerChainGenesisData);
    } else {
        tx1 = api.tx.registrar.registerParathread(rawSpec.para_id, containerChainGenesisData);
    }
    txs.push(
        api.tx.utility.dispatchAs(
            {
                system: { Signed: manager },
            } as any,
            tx1
        )
    );
    if (rawSpec.bootNodes?.length) {
        const tx2 = api.tx.dataPreservers.setBootNodes(rawSpec.para_id, rawSpec.bootNodes);
        txs.push(tx2);
    }
    const tx3 = api.tx.registrar.markValidForCollating(rawSpec.para_id);
    txs.push(tx3);

    return txs;
}

const sleep = (ms: number): Promise<void> => {
    return new Promise((resolve) => setTimeout(resolve, ms));
};
