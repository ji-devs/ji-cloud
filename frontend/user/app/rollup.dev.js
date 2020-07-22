import rust from "@wasm-tool/rollup-plugin-rust";
import serve from "rollup-plugin-serve";
import livereload from "rollup-plugin-livereload";

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
            serverPath: "/js/",
            debug: true,
            watchPatterns: ["src/**", "js/**", "../../_core/rust/src/**", "../../../shared/rust/src/**", "../../../config/rust/src/**"],
            cargoArgs: ["--features", "local quiet"],
            watch: true,
        }),

        serve({
            contentBase: "dist",
            open: true,
            historyApiFallback: true,
        }),

        livereload("dist"),
    ],
};
