if(process.argv.length < 3) {
    console.error("usage is `npm run frontend-dev-files -- [dirname]`");
    process.exit(1);
}

const path = require('path');
const fs = require('fs-extra');

const srcPath = path.resolve("../frontend/_devfiles");
const targetPath = `../frontend/${process.argv[2]}/dist`;


fs.copy(srcPath, targetPath)
  .catch(err => console.error(err))
