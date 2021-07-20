import rust from "@wasm-tool/rollup-plugin-rust";
import nodeResolve from "@rollup/plugin-node-resolve";
import injectProcessEnv from 'rollup-plugin-inject-process-env';

require('dotenv').config({path:__dirname+'/./../../.env'});

const {URL_FRONTEND_RELEASE} = require("../../config/typescript");

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
            serverPath: `${URL_FRONTEND_RELEASE}/${APP_NAME}/js/`,
			cargoArgs: ["--features", "release"],
            debug: false,
        }),
		nodeResolve(),
        injectProcessEnv({
            LOCAL_API_URL: process.env.LOCAL_API_URL,
            LOCAL_UPLOADS_URL: process.env.LOCAL_UPLOADS_URL,
            LOCAL_MEDIA_URL: process.env.LOCAL_MEDIA_URL,
            LOCAL_PAGES_URL: process.env.LOCAL_PAGES_URL,
            LOCAL_PAGES_URL_IFRAME: process.env.LOCAL_PAGES_URL_IFRAME,
            LOCAL_FRONTEND_URL: process.env.LOCAL_FRONTEND_URL,
        }),
    ],
};
