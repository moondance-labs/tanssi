import { execSync } from "child_process";
import { Octokit } from "octokit";
import { readFileSync } from "fs";
import yargs from "yargs";
import path from "path";
import { getCommitAndLabels, getCompareLink } from "./github-utils";
import { blake2AsHex } from "@polkadot/util-crypto";

const BREAKING_CHANGES_LABEL = "D2-breaksapi";
const RUNTIME_CHANGES_LABEL = "B7-runtimenoteworthy";
// `ParachainSystem` is pallet index 1. `authorize_upgrade` is extrinsic index 2.
const DANCEBOX_PREFIX_PARACHAINSYSTEM_AUTHORIZE_UPGRADE = "0x0102";

function capitalize(s) {
  return s[0].toUpperCase() + s.slice(1);
}

function getRuntimeInfo(srtoolReportFolder: string, runtimeName: string) {
  if (runtimeName.includes("-template")) {
    const specVersion = execSync(
      `cat ../container-chains/templates/${runtimeName.split("-template")[0]}/runtime/src/lib.rs | grep 'spec_version: [0-9]*' | tail -1`
    ).toString();
    return {
      name: runtimeName,
      version: /:\s?([0-9A-z\-]*)/.exec(specVersion)[1],
      srtool: JSON.parse(
        readFileSync(path.join(srtoolReportFolder, `./${runtimeName}-srtool-digest.json`)).toString()
      ),
    };
  }
  else {
    const specVersion = execSync(
      `cat ../runtime/${runtimeName}/src/lib.rs | grep 'spec_version: [0-9]*' | tail -1`
    ).toString();
    return {
      name: runtimeName,
      version: /:\s?([0-9A-z\-]*)/.exec(specVersion)[1],
      srtool: JSON.parse(
        readFileSync(path.join(srtoolReportFolder, `./${runtimeName}-srtool-digest.json`)).toString()
      ),
    };
  }
}

// Srtool expects the pallet parachain_system to be at index 1. However just in case we recalculate
function authorizeUpgradeHash(runtimeName: string, srtool: any): string {
  if (runtimeName == "dancebox") {
    return blake2AsHex(
      DANCEBOX_PREFIX_PARACHAINSYSTEM_AUTHORIZE_UPGRADE +
        srtool.runtimes.compressed.blake2_256.substr(2) // remove "0x" prefix
    );
  } else {
    return blake2AsHex(
      DANCEBOX_PREFIX_PARACHAINSYSTEM_AUTHORIZE_UPGRADE +
        srtool.runtimes.compressed.blake2_256.substr(2) // remove "0x" prefix
    );
  }
}

async function main() {
  const argv = yargs(process.argv.slice(2))
    .usage("Usage: npm run ts-node github/generate-release-body.ts [args]")
    .version("1.0.0")
    .options({
      "srtool-report-folder": {
        type: "string",
        describe: "folder which contains <runtime>-srtool-digest.json",
        required: true,
      },
      from: {
        type: "string",
        describe: "previous tag to retrieve commits from",
        required: true,
      },
      to: {
        type: "string",
        describe: "current tag to draft",
        required: true,
      },
      owner: {
        type: "string",
        describe: "Repository owner (Ex: moondance-labs)",
        required: true,
      },
      repo: {
        type: "string",
        describe: "Repository name (Ex: dancebox)",
        required: true,
      },
      runtimes: {
        type: "array",
        describe: "The runtimes for which it needs to be run",
        required: true
      }
    })
    .demandOption(["srtool-report-folder", "from", "to", "runtimes"])
    .help().argv;

  const octokit = new Octokit({
    auth: process.env.GITHUB_TOKEN || undefined,
  });

  const previousTag = argv.from;
  const newTag = argv.to;
  const injectedRuntimes = argv.runtimes;
  
  const runtimes = injectedRuntimes.map((runtimeName) =>
    getRuntimeInfo(argv["srtool-report-folder"], runtimeName as string)
  );

  const moduleLinks = ["polkadot-sdk", "frontier"].map((repoName) => ({
    name: repoName,
    link: getCompareLink(repoName, previousTag, newTag),
  }));

  const { prByLabels } = await getCommitAndLabels(
    octokit,
    argv.owner,
    argv.repo,
    previousTag,
    newTag
  );
  const filteredPr = prByLabels[RUNTIME_CHANGES_LABEL] || [];

  const printPr = (pr) => {
    if (pr.labels.includes(BREAKING_CHANGES_LABEL)) {
      return "⚠️ " + pr.title + " (#" + pr.number + ")";
    } else {
      return pr.title + " (#" + pr.number + ")";
    }
  };

  //

  const template = `${
    runtimes.length > 0
      ? `## Runtimes

${runtimes
  .map(
    (runtime) => `### ${capitalize(runtime.name)}
\`\`\`
✨ spec_version                : ${runtime.version}
🏋 size                        : ${runtime.srtool.runtimes.compressed.size}
#️⃣ sha256                      : ${runtime.srtool.runtimes.compressed.sha256}
#️⃣ blake2-256                  : ${runtime.srtool.runtimes.compressed.blake2_256}
🗳️ proposal (authorizeUpgrade) : ${authorizeUpgradeHash(runtime.name, runtime.srtool)}
\`\`\``
  )
  .join(`\n\n`)}
`
      : ""
  }

## Build information

WASM runtime built using \`${runtimes[0]?.srtool.info.rustc}\`

## Changes

${filteredPr.map((pr) => `* ${printPr(pr)}`).join("\n")}

## Dependency changes

Tanssi: https://github.com/${argv.owner}/${argv.repo}/compare/${previousTag}...${newTag}
${moduleLinks.map((modules) => `${capitalize(modules.name)}: ${modules.link}`).join("\n")}
`;
  console.log(template);
}

main();
