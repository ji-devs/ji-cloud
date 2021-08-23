const { execSync } = require('child_process');
const path = require('path');
const fs = require('fs-extra');

require('dotenv').config({
    path: path.resolve('../../.env')
});

const {APP_NAME} = process.env;

const ALLOWED_APP_NAMES = [
	"user",
	"admin",
	"home",
	"kids",
	"jig/edit",
	"jig/play",
	"legacy/play",
	"module/memory/edit",
	"module/memory/play",
	"module/flashcards/edit",
	"module/flashcards/play",
	"module/card-quiz/edit",
	"module/card-quiz/play",
	"module/matching/edit",
	"module/matching/play",
	"module/poster/edit",
	"module/poster/play",
	"module/cover/edit",
	"module/cover/play",
	"module/video/edit",
	"module/video/play",
	"module/tapping-board/edit",
	"module/tapping-board/play",
	"module/drag-drop/edit",
	"module/drag-drop/play",
	"dev/scratch/001",
	"dev/showcase/001",
]

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
const destDir = `../apps/dist/${APP_NAME}`;
fs.ensureDirSync(path.resolve(destDir));

const srcPathIndex = path.resolve("./dev-index.html");
const destPathIndex = path.resolve(`${destDir}/index.html`);

fs.readFile(srcPathIndex, 'utf-8')
    .then(html => html.replace(/{{APP_NAME}}/g, APP_NAME))
    .then(html => fs.writeFile(destPathIndex, html))
    .catch(err => console.error(err));