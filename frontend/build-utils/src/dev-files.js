const { execSync } = require('child_process');
const path = require('path');
const fs = require('fs-extra');

require('dotenv').config({
    path: path.resolve('../../.env')
});

const {APP_NAME, FRONTEND_DEV_AUTH, FRONTEND_DEV_TOKEN, FRONTEND_DEV_CSRF} = process.env;

const ALLOWED_APP_NAMES = [
	"user",
	"admin",
	"jig/edit",
	"jig/play",
	"module/memory/edit",
	"module/memory/play",
	"module/poster/edit",
	"module/poster/play",
]

if(FRONTEND_DEV_AUTH) {
    if(!FRONTEND_DEV_TOKEN || !FRONTEND_DEV_CSRF) {
        console.error("FRONTEND_DEV_AUTH requires FRONTEND_DEV_TOKEN and FRONTEND_DEV_CSRF in .env");
        process.exit(1);
    }
} else {
    console.error("REQUIRES FRONTEND_DEV_AUTH!");
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
    .then(html => html.replace("{{APP_NAME}}", APP_NAME))
    .then(html => html.replace("{{FRONTEND_DEV_AUTH}}", FRONTEND_DEV_AUTH))
    .then(html => html.replace("{{FRONTEND_DEV_TOKEN}}", FRONTEND_DEV_TOKEN))
    .then(html => html.replace("{{FRONTEND_DEV_CSRF}}", FRONTEND_DEV_CSRF))
    .then(html => fs.writeFile(destPath, html))
    .catch(err => console.error(err));

