require('dotenv').config();

if(!process.env.LOCAL_CDN_DIR || process.env.LOCAL_CDN_DIR === "") {
    console.log("Local CDN: set [LOCAL_CDN_DIR] in .env");
    process.exit(0);
}

const os = require('os');
const path = require('path');
const fs = require('fs');

const localPath = path.resolve(process.env.LOCAL_CDN_DIR);

const express = require('express');
const cors = require('cors');
const serveIndex = require('serve-index');

const app = express();

app.options('*', cors());
app.use(cors());
app.use(express.static(localPath), serveIndex(localPath, {'icons': true}));


//If you change it here - also change it in storybook and app
app.listen(4102, () => console.log('Local CDN Started!'))