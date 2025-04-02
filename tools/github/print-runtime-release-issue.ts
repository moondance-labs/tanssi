import yargs from "yargs";

async function main() {
  const argv = yargs(process.argv.slice(2))
    .usage("Usage: npm run ts-node github/generate-gh-issue-runtime-release.ts [args]")
    .version("1.0.0")
    .options({
      from: {
        type: "string",
        describe: "previous runtime version",
        required: true,
      },
      to: {
        type: "string",
        describe: "next runtime version",
        required: true,
      },
      client: {
        type: "string",
        describe: "current client version",
        required: true,
      },
    })
    .demandOption(["from", "to", "client"])
    .help().argv;

  const previousVersion = argv.from;
  const newVersion = argv.to;
  const lastClientVersion = argv.client;

  const commonTemplate =
    `
## Release
- [ ] Check all proxy types.

### Tanssi-para
- [ ] Branch from master and create branch \`perm-runtime-${newVersion}-para\`
- [ ] Tag \`perm-runtime-${newVersion}-para\` with runtime-${newVersion}-para and push to github
- [ ] NOTE: if this is a hotfix to one of the runtimes, branch from runtime-${previousVersion}-para version
and create perm-runtime-${newVersion}-para
- [ ] Start the github action Publish Runtime Draft
with runtime-${previousVersion}-para => runtime-${newVersion}-para orchestrator-para-only
- [ ] Review the generated Draft and clean a bit the messages if needed (keep it draft)
- [ ] Upgrade stagebox using the system.auhtorizeUpgrade with the hash in the blake-256 section of the release, the using system.applyAuthorizedUpgrade with the runtime wasm
- [ ] When everything is ok, publish the draft release

### Tanssi-solo
- [ ] Branch from master and create branch \`perm-runtime-${newVersion}-starlight\`
- [ ] Tag \`perm-runtime-${newVersion}-starlight\` with runtime-${newVersion}-starlight and push to github
- [ ] NOTE: if this is a hotfix to one of the runtimes, branch from runtime-${previousVersion}-starlight version
and create perm-runtime-${newVersion}-starlight
- [ ] Start the github action Publish Runtime Draft
with runtime-${previousVersion}-starlight => runtime-${newVersion}-starlight orchestrator-solo-only
- [ ] Review the generated Draft and clean a bit the messages if needed (keep it draft)
- [ ] Upgrade stagelight using the system.auhtorizeUpgrade with the hash in the blake-256 section of the release, the using system.applyAuthorizedUpgrade with the runtime wasm
- [ ] When everything is ok, publish the draft release

### Templates
- [ ] Branch from master and create branch \`perm-runtime-${newVersion}-templates\`
- [ ] Tag \`perm-runtime-${newVersion}-templates\` with runtime-${newVersion}-templates and push to github
- [ ] NOTE: if this is a hotfix to one of the runtimes, branch from runtime-${previousVersion}-templates version
and create perm-runtime-${newVersion}-templates
- [ ] Start the github action Publish Runtime Draft
with runtime-${previousVersion}-templates => runtime-${newVersion}-templates templates-only
- [ ] Review the generated Draft and clean a bit the messages if needed (keep it draft)
- [ ] Upgrade stagebox and stagelight containers using the system.auhtorizeUpgrade with the hash in the blake-256 section of the release, the using system.applyAuthorizedUpgrade with the runtime wasm
- [ ] When everything is ok, publish the draft release
  `;

  // Detect if it's a major release or hotfix
  if (newVersion.endsWith("00")) {
    const template =
      `
## Requirements
- [ ] To be manually edited (add pending PRs)

## Pre-Release
- [ ] Check that proxy types are adapted to extrinsics changes (
  read all PR descriptions with B7-runtimenoteworthy)
- [ ] Re-run all extrinsics/hooks benchmarks.

${commonTemplate}

## Post Release
- [ ] Publish the docker runtime image (trigger the github action "Publish Docker runtime tanssi")
  - \`gh workflow run "Publish Runtime Draft" -r 'master' ` +
      `-f from=runtime-${previousVersion}-para -f to=runtime-${newVersion}-para\`
- [ ] Publish the docker runtime image (trigger the github action "Publish Docker runtime containers")
  - \`gh workflow run "Publish Runtime Draft" -r 'master' ` +
      `-f from=runtime-${previousVersion}-templates -f to=runtime-${newVersion}-templates\`
- [ ] Publish the docker runtime image tanssi-solochains (trigger the github action "Publish Docker runtime tanssi-solochain")
  - \`gh workflow run "Publish Runtime Draft" -r 'master' ` +
      `-f from=runtime-${previousVersion}-starlight -f to=runtime-${newVersion}-starlight\`
- [ ] Run the action Upgrade typescript API with ${newVersion}, push and empty commit and merge
- [ ] Once merged, run the Publish Typescript API with the commit of the previously merged branch
- [ ] Create a PR that increment spec version (like #1051) in both containers, tanssi and starlight runtimes
    `;
    console.log(template);
  } else {
    const template = `
## Requirements
- [ ] To be manually edited (add pending PRs)

## Pre-Release
- [ ] Bump spec version to ${newVersion}

${commonTemplate}

## Post Release
- [ ] Publish the docker runtime image (trigger the github action "Publish Docker runtime")
    `;
    console.log(template);
  }
}

main();