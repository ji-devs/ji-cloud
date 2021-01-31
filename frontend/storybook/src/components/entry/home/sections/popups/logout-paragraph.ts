 import "@elements/entry/home/sections/logout-section";
  import "@elements/core/popups/popup-container";
  
export default {
  title: 'Popups',
}


const STR_PEACH = "peach";
const STR_MEDIUM="medium";



export const logoutParagraph = () => {
    return `
<popup-container color="${STR_PEACH}" size="${STR_MEDIUM}">

<logout-section >
     

</logout-section>

</popup-container>
    `
}