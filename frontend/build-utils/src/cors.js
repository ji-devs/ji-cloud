const { execSync } = require("child_process");
const path = require("path");
const sh = require("shelljs");
const cwd = sh.pwd().toString();

const config = require("../../config/constants");

const getBucket = () => {
    switch(process.argv[2]) {
        case "--frontend-release": return config.BUCKET_FRONTEND_RELEASE;
        case "--frontend-sandbox": return config.BUCKET_FRONTEND_SANDBOX;
        case "--uploads-release": return config.BUCKET_UPLOADS_RELEASE;
        case "--uploads-sandbox": return config.BUCKET_UPLOADS_SANDBOX;
        case "--media": return config.BUCKET_MEDIA;
        case "--legacy": return config.BUCKET_LEGACY;
    }

    return null;
};

const getConfig = (bucket) => {
    switch (process.argv[2]) {
        case "--frontend-release":
        case "--frontend-sandbox":
            return "storage-frontend-cors.json";
        case "--uploads-release":
        case "--uploads-sandbox":
            return "storage-uploads-cors.json";
        case "--media":
            return "storage-media-cors.json";
        case "--legacy": 
            return "storage-legacy-cors.json";
    }

    return null;
};

const getAction = () => {
    switch (process.argv[3]) {
        case "--get":
            return "get";
        case "--set":
            return "set";
    }

    return null;
};

const bucket = getBucket();
const configFile = getConfig();
const action = getAction();

if (bucket === null || configFile === null || action === null) {
    console.log("invalid command");
    return;
}

const configPath = path.resolve(cwd, `./${configFile}`);

if (action === "set") {
    execSync(`gsutil cors set ${configPath} gs://${bucket}`, {
        stdio: [0, 1, 2],
    });
} else {
    execSync(`gsutil cors get gs://${bucket}`, { stdio: [0, 1, 2] });
}
