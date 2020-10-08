const {getMediaUrl_UI} = require("../../config/js");
const isDev = process.env["NODE_ENV"] === "development";
const MEDIA_UI = getMediaUrl_UI(isDev);
console.log(`media ui: ${MEDIA_UI}`);


module.exports = {
  purge: [
    '../templates/**/*.html',
  ],
  theme: {
    extend: {
      colors: {
        jiblueLight: '#83aef7',
        jiblueMedium: '#6698ed',
        jiblueDark: '#2b54b8',
        jiblueadd: '#dae7fd',
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
        jigooglegrey: '#f0f1f4',
        jicreatorblue: '#edf7ff',
        jigrey: '#a1a8ad',
        jigreyfocus: '#d8d8d8',
        jiheadergrey: '#4a4a4a',
        jijigbordergrey: '#a1a8ad'

      },

      fontSize: {
        14:'14px',
        18: '18px',

      },
      fontFamily: {
        poppins: 'Poppins',
        sans: 'Roboto',
      },
      width: {
        10: '10px',
        30: '30px',
        68: '68px',
        76: '76px',
        102: '102px',
        112: '112px',
        117: '117px',
        150: '150px',
        160: '160px',
        176: '176px',
        186: '186px',
        190: '190px',
        259: '259px',
        270: '270px',
        272:'272px',
        274: '274px',
        288: '288px',
        296: '296px',
        297: '297px',
        325: '325px',
        360: '360px',
        393: '393px',
        408: '408px',
        480: '480px',
        624: '624px',
        640: '640px',
        763: '763px',
        867: '867px',
        '50p': '50%',
        '1/7': '14.28%'


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
        10: '10px',
        20: '20px',
        24: '24px',
        32: '32px',
        56: '56px',
        64: '64px',
        140: '140px',
        160: '160px',
        185: '185px',
        216: '216px',
        383: '383px',
        537: '537px',
        696: '696px',

      },
      backgroundPosition:{
        'right-center':"265px center"
      },
      maxHeight: {
        284: '284px'
      },
      borderRadius: {
        16: '16px',
        20: '20px',
        36: '36px',
      },
      inset: {

        40:'40px',
        45: '45px',
        10:'10px',
        16:'16px',
        20: '20px',
        '15p':'15%',
        25:'25%',
        '-10': '-10px',
        50:'50px',
        '50p':'50%',
        '70p': '70%',
        70: '70px',
        95: '95px',
        115: '115px',
        160: '160px',
      },
      borderWidth: {
        3: '3px'
      },
      padding:{
        60:'60px',
        80: '80px',
        255: '255px',
      },
      backgroundImage: theme => ({
        'shapes': "url('https://i.ibb.co/g9N7MLy/shapes-1.png')",
        'poster': `url('${MEDIA_UI}/Icn_Module_Poster.svg')`

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
