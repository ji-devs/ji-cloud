import rust from "@wasm-tool/rollup-plugin-rust";
import nodeResolve from "@rollup/plugin-node-resolve";
import commonjs from "@rollup/plugin-commonjs";
import injectProcessEnv from "rollup-plugin-inject-process-env";
import { getEnv } from "./rollup.common.mjs";

import constants from "../config/constants.js";
const { URL_FRONTEND_SANDBOX } = constants;

let { APP_NAME } = process.env;

if (!APP_NAME) {
    console.error("INVALID APP_NAME!");
    process.exit(1);
}

export default {
    input: {
        index: `./crates/entry/${APP_NAME}/Cargo.toml`,
    },
    output: {
        dir: `./dist/${APP_NAME}/js/`,
        format: "es",
        sourcemap: true,
    },
    plugins: [
        rust({
            optimize: {
                release: true,
                rustc: true,
            },
            extraArgs: {
                cargo: ["--features", "sandbox"],
            }
        }),
        nodeResolve(),
        commonjs({
            transformMixedEsModules: true,
        }),
        injectProcessEnv(getEnv()),
    ],
};
