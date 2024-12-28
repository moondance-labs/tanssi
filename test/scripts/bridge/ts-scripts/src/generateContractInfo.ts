import fs from "node:fs";
import path from "node:path";

const run = async () => {
    const NetworkId = process.env.ETH_NETWORK_ID || 11155111;
    const basedir = process.env.contract_dir || "../contracts";
    const DeployInfoFile = path.join(basedir, "broadcast", process.env.deploy_script, `${NetworkId}/run-latest.json`);
    const BuildInfoDir = path.join(basedir, "./out");
    const DestFile = process.argv.length >= 3 ? process.argv[2] : process.env["output_dir"] + "/contracts.json";
    type Contract = {
        [key: string]: ContractInfo;
    };
    const contracts: Contract = {};
    const deploymentInfo = JSON.parse(fs.readFileSync(DeployInfoFile, "utf8"));
    type ContractInfo = {
        abi?: object;
        address?: string;
    };
    for (const transaction of deploymentInfo.transactions) {
        if (transaction.transactionType === "CREATE") {
            const contractName: string = transaction.contractName;
            if (contractName) {
                const contractInfo: ContractInfo = { address: transaction.contractAddress };
                const contractBuildingInfo = JSON.parse(
                    fs.readFileSync(path.join(BuildInfoDir, contractName + ".sol", contractName + ".json"), "utf8")
                );
                contractInfo.abi = contractBuildingInfo.abi;
                contracts[contractName] = contractInfo;
            }
        }
    }
    fs.writeFileSync(DestFile, JSON.stringify({ contracts }, null, 2), "utf8");
};

run()
    .then(() => {
        console.log("Contract File generated successfully");
        process.exit(0);
    })
    .catch((err) => {
        console.error(err);
    });
