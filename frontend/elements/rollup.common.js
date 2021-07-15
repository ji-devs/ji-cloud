import resolve from '@rollup/plugin-node-resolve';
import { terser } from 'rollup-plugin-terser';
import filesize from 'rollup-plugin-filesize';
import alias from '@rollup/plugin-alias';
import injectProcessEnv from 'rollup-plugin-inject-process-env';
import typescript from "rollup-plugin-typescript2";
import commonjs from "@rollup/plugin-commonjs";
import json from '@rollup/plugin-json';

const path = require('path');

const filesizeConfig = {
    showGzippedSize: true,
    showBrotliSize: false,
    showMinifiedSize: false,
};


const projectRootDir = path.resolve(__dirname);


//target should be local, sandbox, or release

export function createConfig(target) {
    const { APP_NAME } = process.env;
    const bundleName = (APP_NAME == null) ? "kitchen-sink" : APP_NAME;

    const input = `./src/_bundles/${bundleName}/imports.ts`;
    const file = `./dist/${bundleName}/custom-elements.js`;
    console.info(`BUNDLING ${bundleName} for ${target}`);

    let plugins = [
        // alias({
        //     entries: {
        //         "@utils": path.resolve(projectRootDir, "./.ts-output/frontend/ts-utils"),
        //         "@frontend-config": path.resolve(projectRootDir, "../config"),
        //         "@project-config": path.resolve(projectRootDir, "./.ts-output/config/typescript/src/lib"),
        //         "@elements": path.resolve(projectRootDir, "./.ts-output/frontend/elements/src"),
        //         "@bundles": path.resolve(projectRootDir, "./.ts-output/frontend/elements/src/_bundles")
        //     }
        // }),

        json(),

        resolve(),

        commonjs(),

        //minifyHTML(),

        typescript({
            tsconfigOverride: {
                include: [input]
            }
        }),

        filesize(filesizeConfig),

        injectProcessEnv({ 
			NODE_ENV: target === "local" ? 'development' : 'production',
            DEPLOY_TARGET: target,
        }),
    ];

    if (target === "production") {
        plugins.push(
            terser({
                output: {
                    comments: false
                }
            })
        );
    }

    return {
        input,
        output: [
            {
                file,
                format: 'es',
                sourcemap: true,
            },
        ],

        context: "window",

        //Could be split out since we re-use it across apps
        //But it's pretty small
        //external: ['lit-html', 'lit-element'],

        plugins,
    }
};
