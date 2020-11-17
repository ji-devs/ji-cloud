require('dotenv').config({ path: '../../.env' });

if(!process.env.LOCAL_CDN_MEDIA_DIR || process.env.LOCAL_CDN_MEDIA_DIR === "") {
    console.log("Local Media server: set [LOCAL_CDN_MEDIA_DIR] in .env");
    process.exit(1);
}

if(!process.env.LOCAL_CDN_MEDIA_PORT || process.env.LOCAL_CDN_MEDIA_PORT === "") {
    console.log("Local Media server: set [LOCAL_CDN_MEDIA_PORT] in .env");
    process.exit(1);
}

if(!process.env.LOCAL_CDN_CSS_PORT || process.env.LOCAL_CDN_CSS_PORT === "") {
    console.log("Local Media server: set [LOCAL_CDN_CSS_PORT] in .env");
    process.exit(1);
}

const path = require('path');

const express = require('express');
const cors = require('cors');
const serveIndex = require('serve-index');

startCdnMedia();
startCdnCss();

function startCdnMedia() {
	const port = parseInt(process.env.LOCAL_CDN_MEDIA_PORT);
	const localPath = path.resolve(process.env.LOCAL_CDN_MEDIA_DIR);

	const app = express();

	app.options('*', cors());
	app.use(cors());
	app.use(express.static(localPath, {cacheControl: false}), serveIndex(localPath, {'icons': true}));


	app.listen(port, () => console.log(`Local CDN for Media Started on port ${port}, serving ${localPath}!`))
}

function startCdnCss() {
	const port = parseInt(process.env.LOCAL_CDN_CSS_PORT);
	const localPath = path.resolve(`../css/dist`);

	const app = express();

	app.options('*', cors());
	app.use(cors());
	app.use(express.static(localPath, {cacheControl: false}), serveIndex(localPath, {'icons': true}));


	app.listen(port, () => console.log(`Local CDN for CSS Started on port ${port}, serving ${localPath}!`))
}