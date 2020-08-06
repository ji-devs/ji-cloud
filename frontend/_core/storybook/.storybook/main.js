const path = require('path');

module.exports = {
  stories: [
    '../src/index.js',
    '../src/stories/**/*.js',
  ],
  addons: [
    '@storybook/addon-notes/register',
    '@storybook/addon-storysource',
  ],
 
};
