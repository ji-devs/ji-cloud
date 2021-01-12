import rust from "@wasm-tool/rollup-plugin-rust";
const {URL_FRONTEND_SANDBOX} = require("../../config/typescript");

let {APP_NAME} = process.env;

if(!APP_NAME) {
    console.error("INVALID APP_NAME!");
    process.exit(1);
}

export default {
    input: {
        index: `./crates/entry/${APP_NAME}/Cargo.toml`,
    },
    output: {
        dir: `./dist/${APP_NAME}/js/`,
        format: "iife",
        sourcemap: true,
    },

    plugins: [
        rust({
            serverPath: `${URL_FRONTEND_SANDBOX}/${APP_NAME}/js/`,
			cargoArgs: ["--features", "sandbox"],
            debug: false,
        }),
    ],
};
