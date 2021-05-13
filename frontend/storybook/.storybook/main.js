const path = require('path');
const LiveReloadPlugin = require('webpack-livereload-plugin');
module.exports = {
  "stories": [
    "../src/**/*.mdx",
    "../src/components/**/*.@(js|jsx|ts|tsx)"
  ],
  "addons": [
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
    Object.assign(options.plugins.find((plugin) => plugin[0].includes('plugin-proposal-decorators'))[1], {
      decoratorsBeforeExport: true,
      legacy: false
    })
    return options;
  }
}

async function makeWebpackFinal(config, { configType }) {
  [".html", ".css"]
    .forEach(ext => {
      if (config.resolve.extensions.indexOf(ext) == -1) {
        console.warn(`${ext} was not in webpack resolve.extensions! adding...`);
        config.resolve.extensions.push(ext);
      }
    });

  //change to absolute path so it will work with files
  //loaded from the outside
  config.resolve.modules = [path.resolve(__dirname, "../node_modules")];

  //Remove HMR (see: https://github.com/storybookjs/storybook/tree/master/app/web-components#user-content-setup-page-reload-via-hmr)
  config.plugins = config.plugins.filter(plugin => plugin.constructor.name != "HotModuleReplacementPlugin");
  config.entry = config.entry.filter(entry => entry.indexOf("webpack-hot-middleware") === -1);
  //Add LiveReload
  config.plugins.push(new LiveReloadPlugin());
  

  return config
}
