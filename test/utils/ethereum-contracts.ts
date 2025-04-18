import type Web3 from "web3";
import fs from "node:fs";
import path from "node:path";
import { ALITH_PRIVATE_KEY, alith } from "@moonwall/util";
import { customWeb3Request } from "@moonwall/util";
import type { AbiItem } from "web3";

export interface Compiled {
    byteCode: string;
    contract: any;
    sourceCode: string;
}

export function getAllContracts(): string[] {
    const contractsPath = path.join(__dirname, "../helpers/compiled/");
    const contracts = fs.readdirSync(contractsPath, { withFileTypes: true });
    // Register all the contract code
    return contracts.filter((dirent) => dirent.isFile()).map((contract) => path.basename(contract.name, ".json"));
}

const contracts: { [name: string]: Compiled } = {};
export function getCompiled(name: string): Compiled {
    if (!fs.existsSync(path.join(__dirname, `../helpers/compiled/${name}.json`))) {
        throw new Error(`Contract name (${name}) doesn't exist in test suite`);
    }
    if (!contracts[name]) {
        try {
            contracts[name] = require(`../helpers/compiled/${name}.json`);
        } catch (e) {
            throw new Error(`Contract name ${name} is not compiled. Please run 'npm run pre-build-contracts`);
        }
    }

    return contracts[name];
}

// Deploy and instantiate a contract with manuel seal
export async function deployContractManualSeal(
    web3: Web3,
    contractByteCode: string,
    contractABI: AbiItem[],
    account: string = alith.address,
    privateKey: string = ALITH_PRIVATE_KEY
) {
    const tx = await web3.eth.accounts.signTransaction(
        {
            from: account,
            data: contractByteCode,
            value: "0x00",
            gasPrice: 10_000_000_000,
            gas: "0x100000",
        },
        privateKey
    );
    await customWeb3Request(web3, "eth_sendRawTransaction", [tx.rawTransaction]);
    const rcpt = await web3.eth.getTransactionReceipt(tx.transactionHash);
    return new web3.eth.Contract(contractABI, rcpt.contractAddress);
}
