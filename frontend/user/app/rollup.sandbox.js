import rust from "@wasm-tool/rollup-plugin-rust";
import {URL_FRONTEND_SANDBOX} from "../../shared/js/frontend/settings";

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
            serverPath: `${URL_FRONTEND_SANDBOX}/${NAME}/js/`,
			cargoArgs: ["--features", "sandbox"],
            debug: false,
        }),
    ],
};
