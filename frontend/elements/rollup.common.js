import resolve from '@rollup/plugin-node-resolve';
import { terser } from 'rollup-plugin-terser';
import filesize from 'rollup-plugin-filesize';
import minifyHTML from 'rollup-plugin-minify-html-literals';
import alias from '@rollup/plugin-alias';

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

    const input = `./.ts-output/elements/src/_bundles/${bundleName}/imports.js`;
    const file = `./dist/${bundleName}/custom-elements.js`;
    console.info(`BUNDLING ${bundleName} for ${target}`);

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

        plugins: [
            alias({
                entries: {
                    "@utils": path.resolve(projectRootDir, "./.ts-output/ts-utils"),
                    "@frontend-config": path.resolve(projectRootDir, "../config"),
                    "@elements": path.resolve(projectRootDir, "./.ts-output/elements/src"),
                    "@bundles": path.resolve(projectRootDir, "./.ts-output/elements/src/_bundles")
                }
            }),
            resolve(),
            //minifyHTML(),
            filesize(filesizeConfig),
            terser({
                output: {
                    comments: false
                }
            }),
        ],
    }
};