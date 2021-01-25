import "@elements/entry/home/TOSORT/column-list";
import "@elements/entry/popup/sections/logout-section";
import "@elements/core/titles/variants/title-section";
 import "@elements/core/buttons/rectangle" ;
 import "@elements/core/popups/popups-template";
export default {
  title: 'Popups',
}


const STR_PEACH = "peach";
const STR_MEDIUM="medium";



export const logoutParagraph = () => {
    return `
<popups-template color="${STR_PEACH}" size="${STR_MEDIUM}">

<logout-section >
     

</logout-section>

</popups-template>
    `
}