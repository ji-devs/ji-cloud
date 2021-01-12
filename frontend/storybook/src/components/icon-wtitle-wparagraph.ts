import "@elements/icon-wtitle-wparagraph";
import { PlainTextButton } from "~/components/plain-text-button";
export default {
  title: 'Homepage Paragraph',
}
const STR_PATH ="PinkSmiley.jpg";
const STR_TITLE = "content";
const STR_PARAGRAPH = "A huge library of activities for the jewish holidays, Hebrew, culture, Tora and many more";
const STR_BUTTONLABEL = "See our templates";
const STR_BLUE = "blue";

export const IconWTitleWParagraph = (STR_PATH,STR_TITLE,STR_PARAGRAPH,STR_BUTTONLABEL) => {
    return `
        <icon-wtitle-wparagraph path="${STR_PATH}" title="${STR_TITLE}" paragraph="${STR_PARAGRAPH}">
            ${PlainTextButton({label:STR_BUTTONLABEL, color: STR_BLUE, bold:false, size:"", italic:false})} 
        </icon-wtitle-wparagraph>
    
    `
}