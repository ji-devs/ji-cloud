import "@elements/icon-wtitle-wparagraph";
import "@elements/homepage-sections/logout-section";
import "@elements/titles/title-section";
import { IconWTitleWParagraph } from "~/components/icon-wtitle-wparagraph";
import { PlainTextButton } from "~/components/plain-text-button";

export default {
  title: 'Homepage Paragraph',
}

 const STR_TITLE="Logout";
const STR_PATH_PinkSmiley="Jiggling_Content@2x.png";
const STR_TITLE_PinkSmiley = "Content";
const STR_PARAGRAPH_PinkSmiley = "A huge library of activities for the jewish holidays, Hebrew, culture, Tora and many more";
const STR_BUTTONLABEL_PinkSmiley = "See our templates";
 const STR_SMALL =""
 

const STR_BLUE = "blue";


export const logoutParagraph = () => {
    return `

    <logout-section>
    <title-section titlecolor="${STR_blue}" title="${STR_TITLE}" size="${STR_SIZELARGE}" slot="title"></title-section>


</logout-section>
    `
}