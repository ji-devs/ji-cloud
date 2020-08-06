console.log("process.cwd() = " + process.cwd());
console.log("__dirname = " + __dirname);

module.exports = {
  purge: [
    './src/**/*.js',
    './src/**/*.ts',
    '../templates/**/*.html',
    //this isn't really valid when running from _core itself
    //but doesn't seem to break anything
    '../../_core/templates/**/*.html',
  ],
  theme: {
    extend: {},
  },
  variants: {},
  plugins: [],
}
