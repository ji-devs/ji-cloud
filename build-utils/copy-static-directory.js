const path = require('path');
const fs = require('fs-extra');


const srcPath = path.resolve("./_static");

const targetPath = path.resolve("./dist");

const isDev = process.argv.length > 2 && process.argv[2] === "--dev";

const srcHtmlFile = `${srcPath}/index.html`;

fs.copy(srcHtmlFile, `${targetPath}/index.html`)
  .catch(err => console.error(err))