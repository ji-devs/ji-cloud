import rust from "@wasm-tool/rollup-plugin-rust";
import serve from "rollup-plugin-serve";
import livereload from "rollup-plugin-livereload";

let {APP_NAME, APP_PORT} = process.env;

if(!APP_NAME) {
    console.error("INVALID APP_NAME!");
    process.exit(1);
}

const path = require('path');

const watchPatterns = [
    `./crates/utils/src/**`,
    `./crates/entry/${APP_NAME}/src/**`,
    `./crates/entry/${APP_NAME}/js/**`,
    "../../.template_output/**", 
    "../../css/dist/**", 
    "../../../shared/rust/src/**", 
    "../../../config/rust/src/**", 
    "../../../config/js/dist/**"
].map(x => path.resolve(x));

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
            cargoArgs: ["--features", "local quiet"],
            watch: true,
        }),

        serve({
            contentBase: `dist/${APP_NAME}`,
            open: true,
            historyApiFallback: true,
            port: APP_PORT, 
        }),

        livereload("dist"),
    ],
};
