import rust from "@wasm-tool/rollup-plugin-rust";
import {URL_FRONTEND_RELEASE} from "../../shared/js/frontend/settings";

const NAME = "user";

export default {
    input: {
        index: "./Cargo.toml",
    },
    output: {
        dir: "dist/js/",
        format: "iife",
        sourcemap: true,
    },
    plugins: [
        rust({
            serverPath: `${URL_FRONTEND_RELEASE}/${NAME}/js/`,
			cargoArgs: ["--features", "release"],
            debug: false,
        }),
    ],
};
