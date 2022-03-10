import rust from "@wasm-tool/rollup-plugin-rust";
import serve from "rollup-plugin-serve";
import livereload from "rollup-plugin-livereload";
import nodeResolve from "@rollup/plugin-node-resolve";
import injectProcessEnv from "rollup-plugin-inject-process-env";
import { getEnv } from "./rollup.common.js";

let { APP_NAME, APP_PORT } = process.env;

if (!APP_NAME) {
    console.error("INVALID APP_NAME!");
    process.exit(1);
}

console.info(`*********************`);
console.info(`** BUILDING ${APP_NAME} **`);
console.info(`*********************`);

const path = require("path");

const watchPatterns = [
    `./crates/entry/**/_common/**`,
    `./crates/utils/**`,
    `./crates/components/**`,
    `./crates/renderer/**`,
    `./crates/entry/${APP_NAME}/**`,
    "../elements/dist/**",
    "../../shared/rust/src/**",
    "../../config/rust/src/**",
    "../../config/js/dist/**",
].map((x) => path.resolve(x));

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
            serverPath: "/js/",
            debug: true,
            watchPatterns,
            // cargoArgs: ["--features", "release"],
            cargoArgs: ["--features", "local quiet"],
            watch: true,
        }),

        nodeResolve(),

        injectProcessEnv(getEnv()),

        serve({
            contentBase: `dist/${APP_NAME}`,
            open: true,
            historyApiFallback: true,
            port: APP_PORT,
            host: '0.0.0.0',
        }),

        livereload("dist"),
    ],
};
