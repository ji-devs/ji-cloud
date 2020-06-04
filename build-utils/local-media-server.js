require('dotenv').config();

if(!process.env.LOCAL_CDN_DIR || process.env.LOCAL_CDN_DIR === "") {
    console.log("Local CDN: set [LOCAL_CDN_DIR] in .env");
    process.exit(0);
}

if(!process.env.LOCAL_CDN_PORT || process.env.LOCAL_CDN_PORT === "") {
    console.log("Local CDN: set [LOCAL_CDN_PORT] in .env");
    process.exit(0);
}

const port = parseInt(process.env.LOCAL_CDN_PORT);
const path = require('path');

const localPath = path.resolve(process.env.LOCAL_CDN_DIR);

const express = require('express');
const cors = require('cors');
const serveIndex = require('serve-index');

const app = express();

app.options('*', cors());
app.use(cors());
app.use(express.static(localPath), serveIndex(localPath, {'icons': true}));


app.listen(port, () => console.log(`Local CDN Started on port ${port}, serving ${localPath}!`))
