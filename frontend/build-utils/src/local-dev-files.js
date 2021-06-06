const { execSync } = require('child_process');
const path = require('path');
const fs = require('fs-extra');

require('dotenv').config({
    path: path.resolve('../../.env')
});

const {APP_NAME, DEV_AUTH, API_TOKEN, API_CSRF} = process.env;

const ALLOWED_APP_NAMES = [
	"user",
	"admin",
	"home",
	"jig/edit",
	"jig/play",
	"legacy/play",
	"module/memory/edit",
	"module/memory/play",
	"module/poster/edit",
	"module/poster/play",
	"module/tapping-board/edit",
	"module/tapping-board/play",
	"dev/scratch/001",
	"dev/showcase/001",
]

if(DEV_AUTH) {
    if(!API_TOKEN || !API_CSRF) {
        console.error("DEV_AUTH requires API_TOKEN and API_CSRF in .env");
        process.exit(1);
    }
} else {
    console.error("REQUIRES DEV_AUTH!");
    process.exit(1);
}

if(!APP_NAME) {
    console.error("requires APP_NAME in env");
    process.exit(1);
}

if(ALLOWED_APP_NAMES.indexOf(APP_NAME) === -1) {
	console.error(`${APP_NAME} is an invalid APP_NAME. Must be one of:`);
	console.error(`${JSON.stringify(ALLOWED_APP_NAMES)}`);
    process.exit(1);
}
//HTML
const srcPath = path.resolve("./dev-index.html");

const destDir = `../apps/dist/${APP_NAME}`;
const destPath = path.resolve(`${destDir}/index.html`);

fs.ensureDirSync(path.resolve(destDir));
fs.readFile(srcPath, 'utf-8')
    .then(html => html.replace(/{{APP_NAME}}/g, APP_NAME))
    .then(html => html.replace(/{{DEV_AUTH}}/g, DEV_AUTH))
    .then(html => html.replace(/{{API_TOKEN}}/g, API_TOKEN))
    .then(html => html.replace(/{{API_CSRF}}/g, API_CSRF))
    .then(html => fs.writeFile(destPath, html))
    .catch(err => console.error(err));

