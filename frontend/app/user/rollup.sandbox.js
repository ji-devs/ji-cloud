import rust from "@wasm-tool/rollup-plugin-rust";
const {URL_FRONTEND_SANDBOX} = require("../../../config/js");

const NAME = "user";

export default {
    input: {
        index: "./Cargo.toml",
    },
    output: {
        //entryFileNames: "[name]-[hash].js",
        entryFileNames: "[name].js",
        dir: "dist/js/",
        format: "iife",
        sourcemap: true,
    },
    plugins: [
        rust({
            serverPath: `${URL_FRONTEND_SANDBOX}/${NAME}/js/`,
			cargoArgs: ["--features", "sandbox"],
            debug: false,
        }),
    ],
};
