console.log("process.cwd() = " + process.cwd());
console.log("__dirname = " + __dirname);

module.exports = {
  purge: [
    './src/html/**/*.js',
    './src/html/**/*.ts',
    '../templates/**/*.html',
    '../../_core/templates/**/*.html',
    './src/html/**/*.jsx',
    './src/html/**/*.tsx',
  ],
  theme: {
    extend: {},
  },
  variants: {},
  plugins: [],
}
