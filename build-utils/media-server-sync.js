const spawn = require('cross-spawn');
const fs = require('fs');
const os = require('os');
const path = require('path');

const cmd = process.argv[2];

const CloudStorageMedia = `gs://ji-cloud-eu/`;

let localPath = (() => {
    switch(os.platform()) {
        case "linux": return `/dropbox/container/Dropbox (Jewish Interactive)/ji-cloud-cdn`;
        default: return `D:\\Dropbox (Jewish Interactive)\\ji-cloud-cdn`;
    }
})();

localPath = path.resolve(localPath);



if(cmd === "--hard") {
    console.log(`Syncing ${cmd}`);
    console.log(`From ${localPath}`);
    console.log(`To ${CloudStorageMedia}`);
    spawn.sync("gsutil", ["-m", "rsync", "-d", "-r", localPath, CloudStorageMedia]);
} else {
    console.log("invalid option!");
}
