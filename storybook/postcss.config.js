const isDev = process.env["NODE_ENV"] === "development";

const REMOTE_STATIC = isDev
        ? 'http://localhost:4103'
        : "https://storage.googleapis.com/ji-cloud-eu";


const REMOTE_UI = `${REMOTE_STATIC}/app/ui`;

const plugins = [
    require('postcss-import'),
    require('postcss-url')({ url: ({url}) => url.replace("%REMOTE_UI%", REMOTE_UI), }),
    require('tailwindcss'),
    require('autoprefixer'),
];

if(!isDev) {

    plugins.push(require('cssnano')({ preset: 'default', }));
}

module.exports = {
    map: isDev,
    plugins
}