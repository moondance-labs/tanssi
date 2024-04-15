import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { MIN_GAS_PRICE, customWeb3Request, generateKeyringPair } from "@moonwall/util";
import { ApiPromise, Keyring } from "@polkadot/api";
import { Signer } from "ethers";
import fs from "fs/promises";
import { getAuthorFromDigest, getAuthorFromDigestRange } from "../../util/author";
import { signAndSendAndInclude, waitSessions } from "../../util/block";
import { createTransfer, waitUntilEthTxIncluded } from "../../util/ethereum";
import { chainSpecToContainerChainGenesisData } from "../../util/genesis_data";
import { getKeyringNimbusIdHex } from "../../util/keys";
import { getHeaderFromRelay } from "../../util/relayInterface";

describeSuite({
    id: "P01",
    title: "Zombie Tanssi Test",
    foundationMethods: "zombie",
    testCases: function ({ it, context }) {
        let paraApi: ApiPromise;
        let relayApi: ApiPromise;
        let container2000Api: ApiPromise;
        let container2001Api: ApiPromise;
        let container2002Api: ApiPromise;
        let blockNumber2002Start;
        let blockNumber2002End;
        let ethersSigner: Signer;
        const sessionPeriod = 10;

        beforeAll(async () => {
            paraApi = context.polkadotJs("Tanssi");
            relayApi = context.polkadotJs("Relay");
            container2000Api = context.polkadotJs("Container2000");
            container2001Api = context.polkadotJs("Container2001");
            container2002Api = context.polkadotJs("Container2002");
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

            const container2002Network = container2002Api.consts.system.version.specName.toString();
            const paraId2002 = (await container2002Api.query.parachainInfo.parachainId()).toString();
            expect(container2002Network, "Container2002 API incorrect").to.contain("container-chain-template");
            expect(paraId2002, "Container2002 API incorrect").to.be.equal("2002");

            // Test block numbers in relay are 0 yet
            const header2000 = await getHeaderFromRelay(relayApi, 2000);
            const header2001 = await getHeaderFromRelay(relayApi, 2001);
            const header2002 = await getHeaderFromRelay(relayApi, 2002);

            expect(header2000.number.toNumber()).to.be.equal(0);
            expect(header2001.number.toNumber()).to.be.equal(0);
            expect(header2002.number.toNumber()).to.be.equal(0);
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
            id: "T03",
            title: "Test assignation did not change",
            test: async function () {
                const currentSession = (await paraApi.query.session.currentIndex()).toNumber();
                // TODO: fix once we have types
                const allCollators = (
                    await paraApi.query.authorityAssignment.collatorContainerChain(currentSession)
                ).toJSON();
                const expectedAllCollators = {
                    orchestratorChain: [
                        getKeyringNimbusIdHex("Collator1000-01"),
                        getKeyringNimbusIdHex("Collator1000-02"),
                        getKeyringNimbusIdHex("Collator2002-01"),
                        getKeyringNimbusIdHex("Collator2002-02"),
                    ],
                    containerChains: {
                        "2000": [getKeyringNimbusIdHex("Collator2000-01"), getKeyringNimbusIdHex("Collator2000-02")],
                        "2001": [getKeyringNimbusIdHex("Collator2001-01"), getKeyringNimbusIdHex("Collator2001-02")],
                    },
                };

                expect(allCollators).to.deep.equal(expectedAllCollators);
            },
        });

        it({
            id: "T04",
            title: "Blocks are being produced on container 2000",
            test: async function () {
                const blockNum = (await container2000Api.rpc.chain.getBlock()).block.header.number.toNumber();
                expect(blockNum).to.be.greaterThan(0);
            },
        });

        it({
            id: "T05",
            title: "Blocks are being produced on container 2001",
            test: async function () {
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
            timeout: 60000,
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
            timeout: 30000,
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
            title: "Test live registration of container chain 2002",
            timeout: 300000,
            test: async function () {
                const keyring = new Keyring({ type: "sr25519" });
                const alice = keyring.addFromUri("//Alice", { name: "Alice default" });

                // Read raw chain spec file
                const spec2002 = await fs.readFile("./specs/template-container-2002.json", "utf8");

                // Before registering container chain 2002, ensure that it has 0 blocks
                // Since the RPC doesn't exist at this point, we need to get that from the relay
                const header2002 = await getHeaderFromRelay(relayApi, 2002);
                expect(header2002.number.toNumber()).to.be.equal(0);
                const registered1 = await paraApi.query.registrar.registeredParaIds();
                // TODO: fix once we have types
                expect(registered1.toJSON().includes(2002)).to.be.false;

                const chainSpec2002 = JSON.parse(spec2002);
                const containerChainGenesisData = chainSpecToContainerChainGenesisData(paraApi, chainSpec2002);
                const tx1 = paraApi.tx.registrar.register(2002, containerChainGenesisData);
                const purchasedCredits = 100000n;
                const requiredBalance = purchasedCredits * 1_000_000n;
                const tx2 = paraApi.tx.servicesPayment.purchaseCredits(2002, requiredBalance);
                const bootNodes = [
                    "/ip4/127.0.0.1/tcp/33051/ws/p2p/12D3KooWSDsmAa7iFbHdQW4X8B2KbeRYPDLarK6EbevUSYfGkeQw",
                ];
                const tx3 = paraApi.tx.dataPreservers.setBootNodes(2002, bootNodes);
                const tx4 = paraApi.tx.registrar.markValidForCollating(2002);
                // Send the batch transaction: [register, purchaseCredits, sudo(setBootNodes), sudo(markValidForCollating)]
                const txBatch = paraApi.tx.utility.batchAll([
                    tx1,
                    tx2,
                    paraApi.tx.sudo.sudo(tx3),
                    paraApi.tx.sudo.sudo(tx4),
                ]);
                await signAndSendAndInclude(txBatch, alice);
                // Check that pending para ids contains 2002
                const registered2 = await paraApi.query.registrar.pendingParaIds();
                const registered3 = await paraApi.query.registrar.registeredParaIds();

                // TODO: fix once we have types
                expect(registered2.toJSON()[0][1].includes(2002)).to.be.true;
                // But registered does not contain 2002 yet
                // TODO: fix once we have types
                expect(registered3.toJSON().includes(2002)).to.be.false;
                // Container chain will be registered after 2 sessions, but because `signAndSendAndInclude` waits
                // until the block that includes the extrinsic is finalized, it is possible that we only need to wait
                // 1 session. So use a callback to wait 1 or 2 sessions.
                await waitSessions(context, paraApi, 2, async () => {
                    const registered = await paraApi.query.registrar.registeredParaIds();
                    // Stop waiting when 2002 is registered
                    return registered.toJSON().includes(2002);
                });
                // Check that registered para ids contains 2002
                const registered5 = await paraApi.query.registrar.registeredParaIds();
                // TODO: fix once we have types
                expect(registered5.toJSON().includes(2002)).to.be.true;

                const blockNum = (await paraApi.rpc.chain.getBlock()).block.header.number.toNumber();
                // Round block number to start of session, sometimes the rpc returns the block number of the next block
                blockNumber2002Start = blockNum - (blockNum % sessionPeriod);
            },
        });

        it({
            id: "T13",
            title: "Blocks are being produced on container 2002",
            timeout: 120000,
            test: async function () {
                // Wait 3 blocks because the next test needs to get a non empty value from
                // container2002Api.query.authoritiesNoting()
                await context.waitBlock(3, "Container2002");
            },
        });

        it({
            id: "T14",
            title: "Test container chain 2002 assignation is correct",
            test: async function () {
                const currentSession = (await paraApi.query.session.currentIndex()).toNumber();
                const paraId = (await container2002Api.query.parachainInfo.parachainId()).toString();
                // TODO: fix once we have types
                const containerChainCollators = (
                    await paraApi.query.authorityAssignment.collatorContainerChain(currentSession)
                ).toJSON().containerChains[paraId];

                const writtenCollators = (await container2002Api.query.authoritiesNoting.authorities()).toJSON();

                expect(containerChainCollators).to.deep.equal(writtenCollators);
            },
        });

        it({
            id: "T15",
            title: "Deregister container chain 2002, collators should move to tanssi",
            timeout: 300000,
            test: async function () {
                const keyring = new Keyring({ type: "sr25519" });
                const alice = keyring.addFromUri("//Alice", { name: "Alice default" });

                const registered1 = await paraApi.query.registrar.registeredParaIds();
                // TODO: fix once we have types
                expect(registered1.toJSON().includes(2002)).to.be.true;

                const tx = paraApi.tx.registrar.deregister(2002);
                await signAndSendAndInclude(paraApi.tx.sudo.sudo(tx), alice);
                // Container chain will be deregistered after 2 sessions, but because `signAndSendAndInclude` waits
                // until the block that includes the extrinsic is finalized, it is possible that we only need to wait
                // 1 session. So use a callback to wait 1 or 2 sessions.
                await waitSessions(context, paraApi, 2, async () => {
                    const registered = await paraApi.query.registrar.registeredParaIds();
                    // Stop waiting if 2002 is no longer registered
                    return !registered.toJSON().includes(2002);
                });
                const blockNum = (await paraApi.rpc.chain.getBlock()).block.header.number.toNumber();
                // Round block number to start of session, sometimes the rpc returns the block number of the next block
                blockNumber2002End = blockNum - (blockNum % sessionPeriod);

                // Check that pending para ids removes 2002
                const registered = await paraApi.query.registrar.registeredParaIds();
                // TODO: fix once we have types
                expect(registered.toJSON().includes(2002)).to.be.false;
            },
        });

        it({
            id: "T16",
            title: "Count number of tanssi collators before and during 2002 chain",
            test: async function () {
                // This test depends on T12 and T15 to set blockNumber2002Start and blockNumber2002End
                // The block range must start and end on session boundaries
                expect(blockNumber2002Start % sessionPeriod).to.be.equal(0);
                expect(blockNumber2002End % sessionPeriod).to.be.equal(0);
                expect(sessionPeriod < blockNumber2002Start).to.be.true;
                expect(blockNumber2002Start < blockNumber2002End).to.be.true;
                const fullRotationBlock = 50;
                // Returns true if a full collator rotation happens inside the inclusive range defined by start and end.
                // If the rotation happens exactly at start or exactly at end, this returns false.
                const fullRotationBetween = (start, end) => {
                    return fullRotationBlock > start && fullRotationBlock < end;
                };

                // Start from block 1 because block 0 has no author
                const blockNumber = 1;
                // Consider 3 cases: full rotation can happen before 2002 is registered, while 2002 is registered, or
                // after 2002 is registered.
                // Locally blockNumber2002Start = 40 but in CI it can be 40 or 50 depending on server specs.
                if (fullRotationBetween(blockNumber, blockNumber2002Start - 1)) {
                    // Before 2002 registration: 4 authors
                    await countUniqueBlockAuthors(paraApi, sessionPeriod, blockNumber, fullRotationBlock - 1, 4);
                    await countUniqueBlockAuthors(
                        paraApi,
                        sessionPeriod,
                        fullRotationBlock,
                        blockNumber2002Start - 1,
                        4
                    );
                    // While 2002 is live: 2 authors (the other 2 went to container chain 2002)
                    await countUniqueBlockAuthors(
                        paraApi,
                        sessionPeriod,
                        blockNumber2002Start,
                        blockNumber2002End - 1,
                        2
                    );
                } else if (fullRotationBetween(blockNumber2002Start, blockNumber2002End - 1)) {
                    // Rotation happened while 2002 was registered
                    // Before 2002 registration: 4 authors
                    await countUniqueBlockAuthors(paraApi, sessionPeriod, blockNumber, blockNumber2002Start - 1, 4);
                    // While 2002 is live: 2 authors (the other 2 went to container chain 2002)
                    await countUniqueBlockAuthors(
                        paraApi,
                        sessionPeriod,
                        blockNumber2002Start,
                        fullRotationBlock - 1,
                        2
                    );
                    await countUniqueBlockAuthors(paraApi, sessionPeriod, fullRotationBlock, blockNumber2002End - 1, 2);
                } else {
                    // Rotation happened at the same time as 2002 was registered, or after 2002 was deregistered
                    // Before 2002 registration: 4 authors
                    await countUniqueBlockAuthors(paraApi, sessionPeriod, blockNumber, blockNumber2002Start - 1, 4);
                    // While 2002 is live: 2 authors (the other 2 went to container chain 2002)
                    await countUniqueBlockAuthors(
                        paraApi,
                        sessionPeriod,
                        blockNumber2002Start,
                        blockNumber2002End - 1,
                        2
                    );
                }
            },
        });

        it({
            id: "T17",
            title: "Count number of tanssi collators after 2002 chain",
            timeout: 120000,
            test: async function () {
                // This test depends on T12 and T15 to set blockNumber2002Start and blockNumber2002End
                const blockNum = (await paraApi.rpc.chain.getBlock()).block.header.number.toNumber();
                if (blockNum < blockNumber2002End + sessionPeriod - 1) {
                    // Need to wait one session because the following blocks don't exist yet
                    await waitSessions(context, paraApi, 1);
                }
                // After 2002 deregistration: 4 authors
                await countUniqueBlockAuthors(
                    paraApi,
                    sessionPeriod,
                    blockNumber2002End,
                    blockNumber2002End + sessionPeriod - 1,
                    4
                );
            },
        });
    },
});

/// Verify that the next `numBlocks` have no more than `numAuthors` different authors
///
/// Concepts: blocks and slots.
/// A slot is a time-based period where one author can propose a block.
/// Block numbers are always consecutive, but some slots may have no block.
/// One session consists of a fixed number of blocks, but a variable number of slots.
///
/// We want to ensure that all the eligible block authors are trying to propose blocks.
///
/// If the authority set changes between `blockStart` and `blockEnd`, this test returns an error.
async function countUniqueBlockAuthors(
    paraApi: ApiPromise,
    sessionPeriod: number,
    blockStart: number,
    blockEnd: number,
    numAuthors: number
) {
    expect(blockEnd, "Called countUniqueBlockAuthors with empty block range").toBeGreaterThan(blockStart);
    // If the expected numAuthors is greater than the session length, it is possible for some authors to never have a
    // chance to produce a block, in that case this test will fail.
    // This test can also fail if the values are close, because collators sometimes fail to produce a block.
    // For optimal results use a value of `numAuthors` that is much smaller than `sessionPeriod`.
    expect(numAuthors).toBeLessThanOrEqual(sessionPeriod);
    // If the authority set changes at any point, the assumption that numAuthors == authorities.len is not valid:
    // we can always have 1 collator assigned to this chain, but if the authority set changes once in the middle of this
    // test, we will see 2 different block authors. We detect that and return an error, the caller is expected to avoid
    // this case by passing a different block range.
    const authoritiesBySession = await fetchAuthoritySetChanges(paraApi, sessionPeriod, blockStart, blockEnd);
    // If there's more than one set of authorities, it means there was a change
    expect(
        authoritiesBySession.size,
        `Authority set did change in the block range passed to countUniqueBlockAuthors, the results will not be consistent. Authority sets: ${formatAuthoritySets(
            authoritiesBySession
        )}`
    ).toBe(1);
    const actualAuthors = [];
    const blockNumbers = [];

    const authors = await getAuthorFromDigestRange(paraApi, blockStart, blockEnd);
    for (let i = 0; i < authors.length; i++) {
        const [blockNum, author] = authors[i];
        blockNumbers.push(blockNum);
        actualAuthors.push(author);
    }

    const uniq = [...new Set(actualAuthors)];

    if (uniq.length > numAuthors || (uniq.length == 1 && numAuthors > 1)) {
        console.error(
            "Mismatch between authorities and actual block authors: authorities: ",
            formatAuthoritySets(authoritiesBySession),
            "",
            actualAuthors,
            ", block numbers: ",
            blockNumbers,
            `uniq.length=${uniq.length}, numAuthors=${numAuthors}`
        );
        expect(false).to.be.true;
    }
}

// Returns the initial set of authorities at `blockStart`, and any different sets of authorities if they changed before
// `blockEnd`, in a map indexed by session number.
async function fetchAuthoritySetChanges(
    paraApi: ApiPromise,
    sessionPeriod: number,
    blockStart: number,
    blockEnd: number
): Promise<Map<number, any>> {
    const authoritiesBySession = new Map<number, any>();
    let lastAuthorities: any = null;

    for (let blockNum = blockStart; blockNum <= blockEnd; blockNum += sessionPeriod) {
        const blockHash = await paraApi.rpc.chain.getBlockHash(blockNum);
        const apiAt = await paraApi.at(blockHash);
        const session = (await apiAt.query.session.currentIndex()).toNumber();
        const authorities = (await apiAt.query.authorityAssignment.collatorContainerChain(session)).toJSON();

        // If this is the first iteration or if the authorities have changed
        if (!lastAuthorities || JSON.stringify(lastAuthorities) !== JSON.stringify(authorities)) {
            authoritiesBySession.set(session, authorities);
        }

        lastAuthorities = authorities;
    }

    return authoritiesBySession;
}

function formatAuthoritySets(authoritiesBySession: Map<number, any>): string {
    let logString = "";

    authoritiesBySession.forEach((authorities, session) => {
        logString += `Session ${session} authorities:\n${JSON.stringify(authorities, null, 4)}`;
    });

    return logString;
}
