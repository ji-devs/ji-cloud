const init = () => {
    if(process.argv.length > 3) {
        //these are all just for local dev mode (real release gets it via automatic cloud service)
        if(process.argv[2] === "--target") {
            switch(process.argv[3]) {
                case "local": {
                    console.log("dev - local mode");
                    process.env["GOOGLE_APPLICATION_CREDENTIALS"] = process.env["GOOGLE_APPLICATION_CREDENTIALS_DEV_SANDBOX"];
                }
                break;
                
                case "sandbox": {
                    console.log("dev - sandbox mode");
                    process.env["GOOGLE_APPLICATION_CREDENTIALS"] = process.env["GOOGLE_APPLICATION_CREDENTIALS_DEV_SANDBOX"];
                }
                
                break;
                case "release": {
                    console.log("dev- release mode");
                    process.env["GOOGLE_APPLICATION_CREDENTIALS"] = process.env["GOOGLE_APPLICATION_CREDENTIALS_DEV_RELEASE"];
                }
                break;
            }
        }
    }

    if(!process.env["GOOGLE_APPLICATION_CREDENTIALS"]) {
        console.warn("Couldn't find env var for GOOGLE_APPLICATION_CREDENTIALS\nIf developing locally, pass --target [local/sandbox/release] or simply `npm run dev`");
    }
}

module.exports = {
    init
};
