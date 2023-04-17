const { readFileSync } = require("fs");
const { spawnSync } = require("child_process");
const { JSDOM } = require("jsdom");
const { applyCustomCodeblocks } = require("./custom_codeblocks.js");

const comrakOutput = spawnSync("./bin/comrak", [
    "--unsafe",
    "--syntax-highlighting",
    "base16-ocean.dark",
    process.argv[2] || ""
]);
if (comrakOutput.status !== 0) {
    process.stderr.write(comrakOutput.stderr);
    process.exit(comrakOutput.status || 1);
}

const dom = new JSDOM(readFileSync("./template.html"));
const document = dom.window.document;

const contentElem = document.querySelector("#content");
contentElem.innerHTML = comrakOutput.stdout;

const headerElem = document.querySelector("h1");
const titleElem = document.querySelector("title");
titleElem.textContent = headerElem.textContent;

applyCustomCodeblocks(dom);

process.stdout.write(dom.serialize());
