module.exports = {
  "stories": [
    '../src/index.js',
    "../src/**/*.stories.mdx",
    "../src/**/*.stories.@(js|jsx|ts|tsx)"
  ],
  "addons": [
    "@storybook/addon-links",
    {
      name: '@storybook/addon-essentials',
      options: {
        backgrounds: false,
        actions: false,
        controls: false
      }
    }
  ],

  webpackFinal: makeWebpackFinal
}

async function makeWebpackFinal(config, {configType}) {
  [".html", ".css"]
    .forEach(ext => {
      if(config.resolve.extensions.indexOf(ext) == -1) {
        console.warn(`${ext} was not in webpack resolve.extensions! adding...`);
        config.resolve.extensions.push(ext);
      }
    });

  return config
}
