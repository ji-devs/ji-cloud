import resolve from "@rollup/plugin-node-resolve";
import { terser } from "rollup-plugin-terser";
import filesize from "rollup-plugin-filesize";
import alias from "@rollup/plugin-alias";
import injectProcessEnv from "rollup-plugin-inject-process-env";
import typescript from "rollup-plugin-typescript2";
import commonjs from "@rollup/plugin-commonjs";
import json from "@rollup/plugin-json";
import minifyHTML from 'rollup-plugin-minify-html-literals';
import dotenv from "dotenv";
import path from 'path';

const __dirname = path.resolve();

dotenv.config({ path: __dirname + "/./../../.env" });

const filesizeConfig = {
    showGzippedSize: true,
    showBrotliSize: false,
    showMinifiedSize: false,
};

const projectRootDir = path.resolve(__dirname);

//target should be local, sandbox, or release

export function createConfig(target) {
    const { APP_NAME } = process.env;
    const bundleName = APP_NAME == null ? "kitchen-sink" : APP_NAME;

    const input = `./src/_bundles/${bundleName}/imports.ts`;
    const file = `./dist/${bundleName}/custom-elements.js`;
    console.info(`BUNDLING ${bundleName} for ${target}`);

    let plugins = [
        // alias({
        //     entries: {
        //         "@utils": path.resolve(projectRootDir, "./.ts-output/frontend/ts-utils"),
        //         "@config": path.resolve(projectRootDir, "../config"),
        //         "@elements": path.resolve(projectRootDir, "./.ts-output/frontend/elements/src"),
        //         "@bundles": path.resolve(projectRootDir, "./.ts-output/frontend/elements/src/_bundles")
        //     }
        // }),

        json(),

        resolve(),

        commonjs({
            transformMixedEsModules: true,
        }),

        typescript({
            tsconfigOverride: {
                include: [input],
            },
        }),

        filesize(filesizeConfig),

        injectProcessEnv(
            // Exclude LOCAL_ values if they're not set (any environment except local)
            Object.fromEntries(
                Object.entries({
                    NODE_ENV: target === "local" ? "development" : "production",
                    DEPLOY_TARGET: target,
                    LOCAL_MEDIA_URL: process.env.LOCAL_MEDIA_URL,
                    LOCAL_UPLOADS_URL: process.env.LOCAL_UPLOADS_URL,
                }).filter(([_, value]) => value !== null && value !== undefined)
            )
        ),
    ];

    if (target !== "local") {
        plugins.push(
            terser({
                output: {
                    comments: false,
                },
            })
        );

        // https://github.com/asyncLiz/minify-html-literals/issues/30
        // plugins.push(minifyHTML());
    }

    return {
        input,
        output: [
            {
                file,
                format: "es",
                sourcemap: true,
            },
        ],

        context: "window",

        //Could be split out since we re-use it across apps
        //But it's pretty small
        //external: ['lit-html', 'lit-element'],

        plugins,
    };
}
