// I don't know what this does
// taken from: https://github.com/storybookjs/storybook/tree/next/examples/html-kitchen-sink

import { addParameters } from '@storybook/html';

const SOURCE_REGEX = /^\(\) => [`'"](.*)['`"]$/;

addParameters({
  a11y: {
    config: {},
    options: {
      checks: { 'color-contrast': { options: { noScroll: true } } },
      restoreScroll: true,
    },
  },
  options: {
    showRoots: true,
  },
  docs: {
    iframeHeight: '200px',
    transformSource: (src) => {
      const match = SOURCE_REGEX.exec(src);
      return match ? match[1] : src;
    },
  },
});