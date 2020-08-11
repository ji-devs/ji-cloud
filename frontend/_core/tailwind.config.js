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
    extend: {
      colors: {
        jiblueLight: '#83aef7',
        jiblueMedium: '#6698ed',
        jiblueDark: '#2b54b8',
        jigreen: '#6eca90',
        jibuttonBlue:'#5590fc',



      },
      fontSize: {
        14:'14px',
        18: '18px',

      },
      fontFamily: {
        azoSans: 'azo-sans-web'
      },
      width: {
        259: '259px',
      },
      borderRadius: {
        20: '20px'
      },
    },
  },
  variants: {
    backgroundColor: ['responsive', 'hover', 'focus', 'active', 'group-hover'],
    border: ['responsive', 'hover', 'focus', 'active', 'group-hover'],
    transitionProperty: ['responsive', 'motion-safe', 'motion-reduce'],
  },
  plugins: [],
}
