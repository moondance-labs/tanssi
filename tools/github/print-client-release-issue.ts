import yargs from "yargs";

async function main() {
  const argv = yargs(process.argv.slice(2))
    .usage("Usage: npm run ts-node github/generate-gh-issue-client-release.ts [args]")
    .version("1.0.0")
    .options({
      from: {
        type: "string",
        describe: "previous client version",
        required: true,
      },
      to: {
        type: "string",
        describe: "next client version",
        required: true,
      },
      chainType: {
          type: "string",
          describe: "type of chain we are building binary for",
          required: true,
      }
    })
    .demandOption(["from", "to"])
    .help().argv;

  const previousVersion = argv.from;
  const newVersion = argv.to;

  const parachainTemplate = `
- [ ] Start the github action Publish Binary Draft with ${previousVersion} => ${newVersion}
(master branch).
- [ ] Review the generated Draft and clean a bit the messages if needed (keep it draft).
- [ ] Start the internal optimized binary build by starting the github action Prepare Optimized Binary Draft with the commit of ${newVersion} (master branch)
- [ ] Update chain-networks stagenet-dancebox, stagebox-container-frontier, stagebox-container-simple, stagelight-collators and stagelight-containers-frontier config.json to include sha-xxxxx built from the optimized binary and pushed to docker
(matching your ${newVersion}-para tag) and increase the config version + 1.
- [ ] Test the new client on stagenet-dancebox.
- [ ] Publish the client release draft.
- [ ] When everything is ok, publish the new docker image: start github action Publish Docker
with ${newVersion}.
`;

  const solochainTemplate = `
- [ ] Start the github action Publish Dancelight Binary Draft with ${previousVersion} => ${newVersion}
(master branch).
- [ ] Review the generated Draft and clean a bit the messages if needed (keep it draft).
- [ ] Start the internal optimized binary build by starting the github action Prepare Optimized Dancelight Binary Draft with the commit of ${newVersion} (master branch)
- [ ] Update chain-networks stagelight config.json to include sha-xxxxx built from the optimized binary and pushed to docker
(matching your ${newVersion} tag) and increase the config version + 1.
- [ ] Test the new client on stagelight.
- [ ] Publish the client release draft.
- [ ] When everything is ok, publish the new docker image: start github action Publish Docker solochain
with ${newVersion}.
  `;

  let chosenTemplate = parachainTemplate;
  if (argv.chainType == "solochain") {
      chosenTemplate = solochainTemplate;
  }

  // Detect if it's a major release or hotfix
  if (newVersion.endsWith(".0")) {
    const template = `
## Requirements
- [ ] To be manually edited (add pending PRs)

## Pre-Release
- [ ] Get that PR approved and merged.
- [ ] Re-run all extrinsics/hooks benchmarks.

## Release
- [ ] If solo-chain, tag master with ${newVersion} and push to github. If parachain, tag master with ${newVersion}-para and push to github.
${chosenTemplate}

## Post Release
- [ ] Bump client version to the next one on master
    `;
    console.log(template);
  } else {
    const template = `
## Requirements
- [ ] To be manually edited (add pending PRs)

## Pre-Release
- [ ] Create branch \`perm-${newVersion}\` against previous client git tag.
- [ ] In the branch \`perm-${newVersion}\`, bump client version to ${newVersion}.

## Release
- [ ] Tag \`perm-${newVersion}\` with ${newVersion} and push to github.
${chosenTemplate}
    `;
    console.log(template);
  }
}

main();