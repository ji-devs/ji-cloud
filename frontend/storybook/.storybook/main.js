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
  ]
}
