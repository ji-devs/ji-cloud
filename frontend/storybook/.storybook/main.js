const path = require("path");
const LiveReloadPlugin = require("webpack-livereload-plugin");

require("dotenv").config({ path: "../../.env" });

const extraEnv = {
    LOCAL_API_URL: process.env.LOCAL_API_URL,
    LOCAL_UPLOADS_URL: process.env.LOCAL_UPLOADS_URL,
    LOCAL_MEDIA_URL: process.env.LOCAL_MEDIA_URL,
    LOCAL_LEGACY_URL: process.env.LOCAL_LEGACY_URL,
    LOCAL_PAGES_URL: process.env.LOCAL_PAGES_URL,
    LOCAL_PAGES_URL_IFRAME: process.env.LOCAL_PAGES_URL_IFRAME,
    LOCAL_FRONTEND_URL: process.env.LOCAL_FRONTEND_URL,
    LOCAL_API_AUTH_OVERRIDE: process.env.LOCAL_API_AUTH_OVERRIDE,
};

module.exports = {
    stories: ["../src/**/*.mdx", "../src/components/**/*.@(js|jsx|ts|tsx)"],
    addons: [
        "@storybook/addon-controls",
        "@storybook/addon-docs",
        /*doesn't help to see source :/
    {
      name: '@storybook/addon-docs',
      options: {
        configureJSX: false,
        babelOptions: {},
      },
    },
    */
    ],

    webpackFinal: makeWebpackFinal,

    //hack to make elements update: https://github.com/storybookjs/storybook/issues/12578
    babel: async (options) => {
        Object.assign(
            options.plugins.find((plugin) =>
                plugin[0].includes("plugin-proposal-decorators")
            )[1],
            {
                decoratorsBeforeExport: true,
                legacy: false,
            }
        );
        return options;
    },
};

async function makeWebpackFinal(config, { configType }) {
    [".html", ".css"].forEach((ext) => {
        if (config.resolve.extensions.indexOf(ext) == -1) {
            console.warn(
                `${ext} was not in webpack resolve.extensions! adding...`
            );
            config.resolve.extensions.push(ext);
        }
    });

    //change to absolute path so it will work with files
    //loaded from the outside
    config.resolve.modules = [path.resolve(__dirname, "../node_modules")];

    //Remove HMR (see: https://github.com/storybookjs/storybook/tree/master/app/web-components#user-content-setup-page-reload-via-hmr)
    config.plugins = config.plugins.filter(
        (plugin) => plugin.constructor.name != "HotModuleReplacementPlugin"
    );
    config.entry = config.entry.filter(
        (entry) => entry.indexOf("webpack-hot-middleware") === -1
    );
    //Add LiveReload
    config.plugins.push(new LiveReloadPlugin());

    //add dotenv (see https://github.com/storybookjs/storybook/issues/12270#issuecomment-755398949)
    const plugin = config.plugins.find((plugin) => {
        return (
            plugin.definitions != null &&
            plugin.definitions["process.env"] != null
        );
    });
    if (plugin) {
        Object.keys(extraEnv).forEach((key) => {
            plugin.definitions["process.env"][key] = JSON.stringify(
                extraEnv[key]
            );
        });
    } else {
        throw new Error(
            "couldn't find definitions plugin (required for setting env overrides)!"
        );
    }
    return config;
}
