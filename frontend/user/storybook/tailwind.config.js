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
        jihoverblue: '#6698ed'
      },
      fontSize: {
        18: '18px',
      },
      fontFamily: {
        azoSans: 'azo-sans-web'
      },
    },
  },
  variants: {},
  plugins: [],
}
