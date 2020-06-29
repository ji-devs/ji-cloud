const { execSync } = require('child_process');
const path = require('path');
const sh = require('shelljs');
const cwd = sh.pwd().toString();


const getBucket = () => {
    switch(process.argv[2]) {
        case "--frontend-release": return "ji-cloud-frontend-origin-eu-001";
        case "--frontend-sandbox": return "ji-cloud-sandbox-frontend-origin-eu-001";
        case "--uploads-release": return "ji-cloud-uploads-origin-eu-001";
        case "--uploads-sandbox": return "ji-cloud-sandbox-uploads-origin-eu-001";
        case "--media": return "ji-cloud-media-origin-eu-001";
    }

    return null;
}

const getConfig = bucket => {
    switch(process.argv[2]) {
        case "--frontend-release": 
        case "--frontend-sandbox": 
            return "storage-frontend-cors.json";
        case "--uploads-release": 
        case "--uploads-sandbox": 
            return "storage-uploads-cors.json";
        case "--media": 
            return "storage-media-cors.json";
    }

    return null;
}

const getAction = () => {
    switch(process.argv[3]) {
        case "--get": return "get";
        case "--set": return "set";
    }

    return null;
}

const bucket = getBucket();
const configFile = getConfig();
const action = getAction();

if(bucket === null || configFile === null || action === null) {
    console.log("invalid command");
    return;
}

const configPath = path.resolve(cwd, configFile);

if(action === "set") {
    execSync(`gsutil cors set ${configPath} gs://${bucket}`, {stdio:[0,1,2]});
} else {
    execSync(`gsutil cors get gs://${bucket}`, {stdio:[0,1,2]});
}

