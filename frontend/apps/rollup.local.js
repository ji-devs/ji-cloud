import rust from "@wasm-tool/rollup-plugin-rust";
import serve from "rollup-plugin-serve";
import livereload from "rollup-plugin-livereload";
import nodeResolve from "@rollup/plugin-node-resolve";
import commonjs from "@rollup/plugin-commonjs";
import injectProcessEnv from "rollup-plugin-inject-process-env";
import { getEnv } from "./rollup.common.js";
import static_files from "rollup-plugin-static-files";
import dev from 'rollup-plugin-dev'
import fp from 'fastify-plugin'
import fs from 'fs'
import APP_ROOTS from '../available-app.mjs';
import path from 'path';

let { APP_NAME, APP_PORT } = process.env;

if (!APP_NAME) {
    console.error("INVALID APP_NAME!");
    process.exit(1);
}

console.info(`*********************`);
console.info(`** BUILDING ${APP_NAME} **`);
console.info(`*********************`);

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

export default [
    {
        input: {
            index: `./empty.js`,
        },
        output: {
            dir: `./dist/static/`,
        },
        plugins: [
            static_files({
                include: ['./static'],
            }),
        ],
    },

    {
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
                serverPath: `/${APP_NAME}/js/`,
                // serverPath: `/js/`,
                debug: true,
                watchPatterns,
                // wasmBindgenArgs: ["--reference-types"],
                cargoArgs: ["--features", "local quiet"],
                watch: true,
            }),

            nodeResolve(),
            commonjs(),

            injectProcessEnv(getEnv()),

            // serve({
            //     contentBase: `dist/`,

            //     open: true,
            //     // home open to `/`
            //     openPage: APP_NAME === "home" ? "/" : `/${APP_NAME}/`,

            //     // `history` refers to the browsers history api, meaning that the application
            //     // uses the history api and needs to serve the same index page for all routes,
            //     // expect for routes that do exist.
            //     historyApiFallback: `/${APP_NAME}/index.html`,

            //     port: APP_PORT,

            //     // enable next line to serve on the network
            //     // host: '0.0.0.0',
            // }),

            dev({
                dirs: [`dist/`],
                port: APP_PORT,
                // enable next line to serve on the network
                host: '0.0.0.0',
                extend: fp(async (server) => {
                    server.setNotFoundHandler((req, res) => {
                        const app = get_app_name(req.url);
                        const file = path.resolve(`./dist/${app}/index.html`);
                        if (fs.existsSync(file)) {
                            const contents = fs.readFileSync(file, "utf-8");
                            res.type('text/html').send(contents);
                        } else {
                            res.code(404).send("File doesn't exist. You might have to build this entry first");
                        }
                    })
                })
            }),

            livereload("dist"),
        ],
    },
];

function get_app_name(url) {
    // remove leading `/`
    url = url.substring(1);
    for (const appRoot of APP_ROOTS) {
        if (url.startsWith(appRoot)) {
            return appRoot;
        }
    }
    // default to home
    return "home";
}
