const {getMediaUrl_UI} = require("../../../config/js");

const isDev = process.env["NODE_ENV"] === "development";

const MEDIA_UI = getMediaUrl_UI(isDev);

const plugins = [
    require('postcss-import'),
    require('postcss-url')({ url: ({url}) => url.replace("%MEDIA_UI%", MEDIA_UI), }),
    require('tailwindcss')('../../_core/tailwind.config.js'),
    require('autoprefixer'),
];

if(!isDev) {

    plugins.push(require('cssnano')({ preset: 'default', }));
}

module.exports = {
    map: isDev,
    plugins
}
