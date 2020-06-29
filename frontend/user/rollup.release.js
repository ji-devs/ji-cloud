import rust from "@wasm-tool/rollup-plugin-rust";
import {FRONTEND_SERVER_RELEASE} from "../../shared/js/frontend/settings";

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
            serverPath: `${FRONTEND_SERVER_RELEASE}/${NAME}/js/`,
			cargoArgs: ["--features", "release"],
            debug: false,
        }),
    ],
};
