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
        jinumberblue: '#c4dbfe',
        jigreen: '#61D592',
        jibuttonBlue:'#5590fc',
        jibackgroundBlue: "#e6f0ff",
        jibluelighter: "#a6c6f8",
        jiborderGrey: "#e5e7ef",
        jibackgroundGrey:'#f8f9fd',
        jigreyinputborder:'#d3d3d3;',
        jierrorred: '#e36486',
        jiyellowbackground: '#fdf5d0',
        jiyellowstar:'#fccd63',
        jigreyicon: '#d6d8de',
        jiorange:'#fc7551',
        jititleorange: '#fd6220',
        jisideblue:'#def4ff',
        jilogingrey: '#f7f7f7',
        jisignupgrey: '#dee1eb',
        jidarkgrey: '#e2e5eb',
        jibuttongreen: '#72cb91',
        jiimagegreen: '#6eca90',
      },


      fontSize: {
        14:'14px',
        18: '18px',

      },
      fontFamily: {
        poppins: 'Poppins',
      },
      width: {
        10: '10px',
        30: '30px',
        76: '76px',
        112: '112px',
        117: '117px',
        150: '150px',
        176: '176px',
        190: '190px',
        259: '259px',
        272:'272px',
        274: '274px',
        288: '288px',
        297: '297px',
        393: '393px',
        480: '480px',
        624: '624px',
        763: '763px',
        867: '867px',
        '50p': '50%'

      },
      maxWidth: {
        480: '480px'
      },
      minWidth: {
        112: '112px',
        297: '297px',
        300: '300px',
      },
      height: {
        216: '216px',
        185: '185px',
        696: '696px',
        10: '10px',
        20: '20px',
        32: '32px',
        56: '56px',
        64: '64px',

      },
      backgroundPosition:{
        'right-center':"265px center"
      },
      maxHeight: {
        284: '284px'
      },
      borderRadius: {
        16: '16px',
        20: '20px'
      },
      inset: {
        40:'40px',
        10:'10px',
        16:'16px',
        25:'25%',
        '-10': '-10px',
        50:'50px',
        '50p':'50%',
        '70p': '70%',
        20: '20px',
        160: '160px',
      },
      borderWidth: {
        3: '3px'
      },
      padding:{
        60:'60px',
        80: '80px'
      },
      backgroundImage: theme => ({
        'shapes': "url('https://i.ibb.co/g9N7MLy/shapes-1.png')",

        })
    },
  },
  variants: {
    backgroundColor: ['responsive', 'hover', 'focus', 'active', 'group-hover'],
    border: ['responsive', 'hover', 'focus', 'active', 'group-hover'],
    transitionProperty: ['responsive', 'motion-safe', 'motion-reduce'],
  },
  plugins: [],
}
