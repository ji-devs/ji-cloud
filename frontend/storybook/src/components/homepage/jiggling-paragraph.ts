import "@elements/icon-wtitle-wparagraph";
import "@elements/homepage-sections/jiggling-section";
import "@elements/homepage-sections/title-paragraph";
import { IconWTitleWParagraph } from "~/components/icon-wtitle-wparagraph";
import { PlainTextButton } from "~/components/plain-text-button";
export default {
  title: 'Homepage Paragraph',
}

const STR_title="Why Ji?";
const STR_PATH_PinkSmiley="PinkSmiley.jpg";
const STR_TITLE_PinkSmiley = "Content";
const STR_PARAGRAPH_PinkSmiley = "A huge library of activities for the jewish holidays, Hebrew, culture, Tora and many more";
const STR_BUTTONLABEL_PinkSmiley = "See our templates";
const STR_COLORTITLE_PinkSmiley = "#fd6b71";

 const STR_PATH_BlueWheel="BlueWheel.jpg";
const STR_TITLE_BlueWheel = "Create";
const STR_PARAGRAPH_BlueWheel = "Create your own activities, Teach your class to create their own games. The most fun way to learn something new.";
const STR_BUTTONLABEL_BlueWheel = "Try it for free";
const STR_COLORTITLE_BlueWheel = "#2040a3";

 const STR_PATH_GreenRectangle="PinkSmiley.jpg";
const STR_TITLE_GreenRectangle = "Customize";
const STR_PARAGRAPH_GreenRectangle = "Easily, saving time way. Customize our templates for your needs. ";
const STR_BUTTONLABEL_GreenRectangle = "See our templates";
const STR_COLORTITLE_GreenRectangle = "#46ba6f";

 const STR_PATH_YellowSquare="YellowSquare.jpg";
const STR_TITLE_YellowSquare = "Community";
const STR_PARAGRAPH_YellowSquare = "Meet X users around the world. See who plays now. Meet other teachers."  ;
const STR_BUTTONLABEL_YellowSquare = "Get inspired";
const STR_COLORTITLE_YellowSquare = "#fea559";
 
const STR_PATH_BlueTriangle="BlueTriangle.jpg";
const STR_TITLE_BlueTriangle = "Classroom";
const STR_PARAGRAPH_BlueTriangle= "track your students journey, manage your lessons, See which activities are more successful.";
const STR_BUTTONLABEL_BlueTriangle = "Manage your class";
const STR_COLORTITLE_BlueTriangle = "#6ca1fc";


const STR_BLUE = "blue";


export const JigglingParagraph = () => {
    return `

    <jiggling-section>

    <title-paragraph  title="${STR_title}" slot="title"></title-paragraph>
    
    <div slot="icon-title-paragraph">
        ${IconWTitleWParagraph({path:STR_PATH_PinkSmiley,title:STR_TITLE_PinkSmiley, paragraph:STR_PARAGRAPH_PinkSmiley, buttonlabel:STR_BUTTONLABEL_PinkSmiley, colorTitle:STR_COLORTITLE_PinkSmiley,colorButton:STR_BLUE})}
    </div>
    <div slot="icon-title-paragraph">
    ${IconWTitleWParagraph({path:STR_PATH_BlueWheel,title:STR_TITLE_BlueWheel, paragraph:STR_PARAGRAPH_BlueWheel, buttonlabel:STR_BUTTONLABEL_BlueWheel, colorTitle:STR_COLORTITLE_BlueWheel,colorButton:STR_BLUE})}
</div>
<div slot="icon-title-paragraph">
${IconWTitleWParagraph({path:STR_PATH_GreenRectangle,title:STR_TITLE_GreenRectangle, paragraph:STR_PARAGRAPH_GreenRectangle, buttonlabel:STR_BUTTONLABEL_GreenRectangle, colorTitle:STR_COLORTITLE_GreenRectangle,colorButton:STR_BLUE})}
</div>
<div slot="icon-title-paragraph">
${IconWTitleWParagraph({path:STR_PATH_YellowSquare,title:STR_TITLE_YellowSquare, paragraph:STR_PARAGRAPH_YellowSquare, buttonlabel:STR_BUTTONLABEL_YellowSquare, colorTitle:STR_COLORTITLE_YellowSquare,colorButton:STR_BLUE})}
</div>
<div slot="icon-title-paragraph">
${IconWTitleWParagraph({path:STR_PATH_BlueTriangle,title:STR_TITLE_BlueTriangle, paragraph:STR_PARAGRAPH_BlueTriangle, buttonlabel:STR_BUTTONLABEL_BlueTriangle, colorTitle:STR_COLORTITLE_BlueTriangle,colorButton:STR_BLUE})}
</div>
</jiggling-section>
    `
}