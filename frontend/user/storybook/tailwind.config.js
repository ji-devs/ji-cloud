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
    extend: {
      colors: {
        jiblue: '#83aef7',
      },
      fontSize: {
        18: '18px',
      },
      fontFamily: {
        azoSans: 'AzoSans'
      },
    },
  },
  variants: {},
  plugins: [],
}
