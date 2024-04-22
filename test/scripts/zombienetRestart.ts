import * as ps from "ps-node";
import { exec, spawn, execSync } from "child_process";
import { readFileSync, writeFileSync, readlinkSync, unlinkSync } from "fs";
import yargs from "yargs";
import { hideBin } from "yargs/helpers";
import inquirer from "inquirer";

const getEnvVariables = (pid: number) => {
    const envData = readFileSync(`/proc/${pid}/environ`).toString();
    return envData.split("\0").filter(Boolean);
};

const getCwd = (pid: number) => {
    return readlinkSync(`/proc/${pid}/cwd`);
};

const targetProcessNames = ["tanssi-node", "container-chain-simple-node", "container-chain-frontier-node", "polkadot"];
const pattern = targetProcessNames.join("|");

const fetchProcesses = async () => {
    const cmd = `ps aux | grep -E "${pattern}"`;
    const { stdout } = await execPromisify(cmd);
    return stdout
        .split("\n")
        .filter((line) => line && !line.includes("grep -E"))
        .map((line) => {
            const parts = line.split(/\s+/);
            const pid = parts[1];
            const command = parts.slice(10).join(" ");
            return {
                name: `PID: ${pid}, Command: ${command}`,
                value: pid,
            };
        });
};

const execPromisify = (command: string) => {
    return new Promise<{ stdout: string; stderr: string }>((resolve, reject) => {
        exec(command, (error, stdout, stderr) => {
            if (error) {
                reject(error);
            } else {
                resolve({ stdout, stderr });
            }
        });
    });
};

yargs(hideBin(process.argv))
    .usage("Usage: $0 <command> [options]")
    .version("1.0.0")

    .command(
        "restart",
        "Restart a process by its PID",
        (yargs) => {
            return yargs
                .option("pid", {
                    describe: "Process ID of the target process",
                    type: "number",
                    demandOption: false,
                })
                .option("edit-cmd", {
                    describe: "Edit the command before restarting the process",
                    type: "boolean",
                })
                .option("wait-ms", {
                    describe: "Delay (in milliseconds) before restarting the process",
                    type: "number",
                    default: 0,
                });
        },
        async (argv) => {
            let pid = argv.pid as number;

            if (!pid) {
                const processes = await fetchProcesses();
                if (processes.length === 0) {
                    console.error("No matching processes found. Exiting...");
                    process.exit(1);
                }

                const { selectedPid } = await inquirer.prompt([
                    {
                        type: "list",
                        name: "selectedPid",
                        message: "Select a process to restart:",
                        choices: processes,
                        pageSize: 15, // Increase this number as needed
                    },
                ]);

                pid = Number(selectedPid);
            }

            // Get process details by PID
            ps.lookup({ pid: pid }, (err, resultList) => {
                if (err) {
                    throw new Error(err);
                }

                const processInfo = resultList[0];

                if (processInfo) {
                    let { command, arguments: args } = processInfo;

                    if (argv["edit-cmd"]) {
                        const tempFilePath = execSync("mktemp /tmp/zombienet-restart-cmd-XXXXXX").toString().trim();
                        writeFileSync(tempFilePath, `${command} ${args.join(" ")}`);

                        const editor = process.env.EDITOR || "vim"; // Default to 'vim' if EDITOR is not set
                        execSync(`${editor} ${tempFilePath}`, { stdio: "inherit" });

                        const modifiedCommand = readFileSync(tempFilePath, "utf-8").trim().split(" ");
                        command = modifiedCommand[0];
                        args = modifiedCommand.slice(1);

                        // Delete the temporary file
                        unlinkSync(tempFilePath);
                    }

                    console.log(`Command: ${command}`);
                    console.log(`Arguments: ${args.join(" ")}`);

                    // Fetch environment variables, CWD, etc.
                    const envVariables = getEnvVariables(pid);
                    const cwd = getCwd(pid);
                    console.log(`Environment Variables: \n${envVariables.join("\n")}`);
                    console.log(`Current Working Directory: ${cwd}`);

                    // Kill the process
                    exec(`kill -9 ${pid}`, (err) => {
                        if (err) {
                            console.error(`Failed to kill process with ID ${pid}.`, err);
                            return;
                        }

                        console.log(`Process with ID ${pid} has been killed.`);

                        setTimeout(() => {
                            // Restart the process in the current terminal with its original environment variables and cwd
                            const child = spawn(command, args, {
                                stdio: "inherit",
                                cwd: cwd,
                                env: Object.fromEntries(envVariables.map((e) => e.split("=", 2))),
                            });

                            ["SIGINT", "SIGTERM"].forEach((signal) => {
                                process.on(signal, () => {
                                    console.log("zombienetRestart: got ", signal);
                                    if (child) {
                                        child.kill(signal);
                                    }
                                    process.exit();
                                });
                            });
                        }, argv["wait-ms"]);
                    });
                } else {
                    console.log(`Process not found with ID ${pid}.`);
                }
            });
        }
    )

    .command(
        "list",
        "List processes with specified names",
        () => {},
        async () => {
            const processes = await fetchProcesses();
            if (processes.length) {
                console.log("Matching Processes:");
                processes.forEach((process) => {
                    console.log(process.name);
                });
            } else {
                console.log("No matching processes found.");
            }
        }
    )
    .parse();
