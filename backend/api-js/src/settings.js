const init = () => {

    if(process.argv.length > 3) {
        if(process.argv[2] === "--target") {
            if(process.argv[3] === "sandbox") {
                console.log("dev - sandbox mode");
                process.env["GOOGLE_APPLICATION_CREDENTIALS"] = process.env["GOOGLE_APPLICATION_CREDENTIALS_DEV_SANDBOX"];
            } else if(process.argv[3] === "release") {
                console.log("dev- release mode");
                process.env["GOOGLE_APPLICATION_CREDENTIALS"] = process.env["GOOGLE_APPLICATION_CREDENTIALS_DEV_RELEASE"];
            } else {
                throw new Error("unknown --target!");
            }
        }
    }
}

module.exports = {
    init
};
