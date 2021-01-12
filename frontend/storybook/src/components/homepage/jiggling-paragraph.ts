import "@elements/icon-wtitle-wparagraph";
import "@elements/homepage-sections/jiggling-section";
import { IconWTitleWParagraph } from "~/components/icon-wtitle-wparagraph";
import { PlainTextButton } from "~/components/plain-text-button";
export default {
  title: 'Homepage Paragraph',
}
const STR_PATH ="PinkSmiley.jpg";
const STR_TITLE = "Content";
const STR_PARAGRAPH = "A huge library of activities for the jewish holidays, Hebrew, culture, Tora and many more";
const STR_BUTTONLABEL = "See our templates";
const STR_BLUE = "blue";


export const JigglingParagraph = () => {
    return `

    <jiggling-section>
    <plain-black slot="title"></plain-black>
    <div slot="icon-title-paragraph">
        ${IconWTitleWParagraph({path:STR_PATH,title:STR_TITLE, paragraph:STR_PARAGRAPH, buttonlabel:STR_BUTTONLABEL, color:STR_BLUE})}
    </div>
    <icon-wtitle-wparagraph path="${STR_PATH}" title="${STR_TITLE}" paragraph="${STR_PARAGRAPH}" slot="icon-title-paragraph">
        ${PlainTextButton({label:STR_BUTTONLABEL, color: STR_BLUE, bold:false, size:"", italic:false})} 
    </icon-wtitle-wparagraph>
    <icon-wtitle-wparagraph path="${STR_PATH}" title="${STR_TITLE}" paragraph="${STR_PARAGRAPH}" slot="icon-title-paragraph">
        ${PlainTextButton({label:STR_BUTTONLABEL, color: STR_BLUE, bold:false, size:"", italic:false})} 
    </icon-wtitle-wparagraph>
    <icon-wtitle-wparagraph path="${STR_PATH}" title="${STR_TITLE}" paragraph="${STR_PARAGRAPH}" slot="icon-title-paragraph">
        ${PlainTextButton({label:STR_BUTTONLABEL, color: STR_BLUE, bold:false, size:"", italic:false})} 
    </icon-wtitle-wparagraph>
</jiggling-section>
    `
}