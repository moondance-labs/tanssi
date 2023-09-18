import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { ApiPromise, Keyring } from "@polkadot/api";
import { u8aToHex, stringToHex } from "@polkadot/util";
import { decodeAddress } from "@polkadot/util-crypto";
import { getAuthorFromDigest } from "../../util/author";
import { signAndSendAndInclude, waitSessions } from "../../util/block";
import { getKeyringNimbusIdHex } from "../../util/keys";
import { getHeaderFromRelay } from "../../util/relayInterface";
import fs from "fs/promises";
import * as http from 'http';
import * as https from 'https';

describeSuite({
    id: "ZM01",
    title: "Zombie Tanssi Metrics Test",
    foundationMethods: "zombie",
    testCases: function ({ it, context }) {
        let paraApi: ApiPromise;
        let relayApi: ApiPromise;
        let container2000Api: ApiPromise;

        beforeAll(async () => {
            paraApi = context.polkadotJs("Tanssi");
            relayApi = context.polkadotJs("Relay");
            container2000Api = context.polkadotJs("Container2000");

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

            // Test block numbers in relay are 0 yet
            const header2000 = await getHeaderFromRelay(relayApi, 2000);

            expect(header2000.number.toNumber()).to.be.equal(0);
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
                        getKeyringNimbusIdHex("Collator1000-03"),
                    ],
                    containerChains: {
                        "2000": [getKeyringNimbusIdHex("Collator2000-01"), getKeyringNimbusIdHex("Collator2000-02")],
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
            id: "T08",
            title: "Test author noting is correct for both containers",
            timeout: 60000,
            test: async function () {
                const assignment = await paraApi.query.collatorAssignment.collatorContainerChain();
                const paraId2000 = await container2000Api.query.parachainInfo.parachainId();

                // TODO: fix once we have types
                const containerChainCollators2000 = assignment.containerChains.toJSON()[paraId2000.toString()];

                await context.waitBlock(3, "Tanssi");
                const author2000 = await paraApi.query.authorNoting.latestAuthor(paraId2000);

                expect(containerChainCollators2000.includes(author2000.toJSON().author)).to.be.true;
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
            },
        });

        it({
            id: "T12",
            title: "Test metrics: deregister container chain and metrics should stop",
            timeout: 300000,
            test: async function () {
                const keyring = new Keyring({ type: "sr25519" });
                const alice = keyring.addFromUri("//Alice", { name: "Alice default" });

                // Create an agent to keep the HTTP connection alive (optional)
                const agent = new http.Agent({ keepAlive: true, keepAliveMsecs: 300000 });
                expect(await checkUrl('http://127.0.0.1:27124/metrics', agent)).to.be.true;
                expect(await checkUrl('http://127.0.0.1:27125/metrics', agent)).to.be.true;

                const registered1 = await paraApi.query.registrar.registeredParaIds();
                // TODO: fix once we have types
                expect(registered1.toJSON().includes(2000)).to.be.true;

                const tx = paraApi.tx.registrar.deregister(2000);
                await signAndSendAndInclude(paraApi.tx.sudo.sudo(tx), alice);
                await waitSessions(context, paraApi, 2);
                const blockNum = (await paraApi.rpc.chain.getBlock()).block.header.number.toNumber();

                // Check that pending para ids removes 2000
                const registered = await paraApi.query.registrar.registeredParaIds();
                // TODO: fix once we have types
                expect(registered.toJSON().includes(2000)).to.be.false;

                expect(await checkUrl('http://127.0.0.1:27124/metrics', agent)).to.be.false;
                expect(await checkUrl('http://127.0.0.1:27125/metrics', agent)).to.be.false;
            },
        });
    },
});

async function directoryExists(directoryPath) {
    try {
        await fs.access(directoryPath, fs.constants.F_OK);
        return true;
    } catch (err) {
        return false;
    }
}

/// Returns the /tmp/zombie-52234... path
function getTmpZombiePath() {
    const logFilePath = process.env.MOON_MONITORED_NODE;

    if (logFilePath) {
        const lastIndex = logFilePath.lastIndexOf("/");
        return lastIndex !== -1 ? logFilePath.substring(0, lastIndex) : null;
    }

    // Return null if the environment variable is not set
    return null;
}

// Define an async function to check if a URL returns HTTP 200
async function checkUrl(url: string, agent?: http.Agent | https.Agent): Promise<boolean> {
    // Choose the appropriate module based on the URL (http or https)
    const client = url.startsWith('https') ? https : http;

    const requestOptions = {
        agent,
    };
  
    return new Promise<boolean>((resolve, reject) => {
      // Send an HTTP GET request to the URL
      client.get(url, requestOptions, (response) => {
        if (response.statusCode === 200) {
            console.log("checkUrl: ", url, response.statusCode);
          resolve(true);
        } else {
            console.log("checkUrl: ", url, response.statusCode);
          resolve(false);
        }
      }).on('error', (error) => {
        console.log("checkUrl: ", url, error);

        resolve(false);
      });
    });
}