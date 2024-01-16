import { writeFileSync, copyFileSync, readFileSync } from "fs";
import path from "path";

async function main() {
    // console.log("Loading package.json");

    // const pck = JSON.parse(readFileSync(path.join(process.cwd(), "package.json"), "utf-8"));
    const buildPath = `${process.env.PWD}/build`;

    // pck.scripts = {};
    // pck.private = false;
    // pck.type = "module";
    // pck.files = ["**/*", "!**/tsconfig.tsbuildinfo", "!**/*.tgz"];

    // console.log(`Writing ${buildPath}/package.json`);
    // writeFileSync(`${buildPath}/package.json`, JSON.stringify(pck, null, 2));
    // copyFileSync("README.md", `${buildPath}/README.md`);

    // console.log(`Copy ${buildPath}/README.md`);

    // Copy empty files for CommonJS modules
    copyFileSync("./src/index.cjs", `${buildPath}/index.cjs`);
    copyFileSync("./src/index.cjs", `${buildPath}/flashbox/index.cjs`);
    console.log(`Done postbuild`);
}

main()
    .catch((error) => {
        console.error(error);
        process.exit(1);
    })
    .then(() => {
        process.exit(0);
    });
