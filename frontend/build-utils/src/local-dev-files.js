import path from "path";
import fs from "fs-extra";
import ALLOWED_APP_NAMES from "../../available-app.mjs";
import dotenv from "dotenv";

dotenv.config({
    path: path.resolve("../../.env"),
});

const { APP_NAME } = process.env;

if (!APP_NAME) {
    console.error("requires APP_NAME in env");
    process.exit(1);
}

if (ALLOWED_APP_NAMES.indexOf(APP_NAME) === -1) {
    console.error(`${APP_NAME} is an invalid APP_NAME. Must be one of:`);
    console.error(`${JSON.stringify(ALLOWED_APP_NAMES)}`);
    process.exit(1);
}
//HTML
const destDir = `../apps/dist/${APP_NAME}`;
fs.ensureDirSync(path.resolve(destDir));

const srcPathIndex = path.resolve("./dev-index.html");
const destPathIndex = path.resolve(`${destDir}/index.html`);

fs.readFile(srcPathIndex, "utf-8")
    .then((html) => html.replace(/{{APP_NAME}}/g, APP_NAME))
    .then((html) => fs.writeFile(destPathIndex, html))
    .catch((err) => console.error(err));
