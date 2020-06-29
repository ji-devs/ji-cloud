const isDev = process.env["NODE_ENV"] === "development";

const REMOTE_STATIC = isDev
        ? 'http://localhost:4102'
        : "https://media.jicloud.org";


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
