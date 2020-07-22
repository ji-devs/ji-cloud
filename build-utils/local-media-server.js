require('dotenv').config({ path: '../config/.env' });

if(!process.env.LOCAL_CDN_MEDIA_DIR || process.env.LOCAL_CDN_MEDIA_DIR === "") {
    console.log("Local Media server: set [LOCAL_CDN_MEDIA_DIR] in .env");
    process.exit(1);
}

if(!process.env.LOCAL_CDN_MEDIA_PORT || process.env.LOCAL_CDN_MEDIA_PORT === "") {
    console.log("Local Media server: set [LOCAL_CDN_MEDIA_PORT] in .env");
    process.exit(1);
}

const port = parseInt(process.env.LOCAL_CDN_MEDIA_PORT);
const path = require('path');

const localPath = path.resolve(process.env.LOCAL_CDN_MEDIA_DIR);

const express = require('express');
const cors = require('cors');
const serveIndex = require('serve-index');

const app = express();

app.options('*', cors());
app.use(cors());
app.use(express.static(localPath), serveIndex(localPath, {'icons': true}));


app.listen(port, () => console.log(`Local CDN Started on port ${port}, serving ${localPath}!`))
