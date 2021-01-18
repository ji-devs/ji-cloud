import "@elements/icon-wtitle-wparagraph";
import "@elements/homepage-sections/jiggling-section";
import "@elements/titles/title-section";
import { IconWTitleWParagraph } from "~/components/icon-wtitle-wparagraph";
import { PlainTextButton } from "~/components/plain-text-button";

export default {
  title: 'Homepage Paragraph',
}

const STR_PURPLE = "purple";
const STR_TITLE="Why Ji?";
const STR_PATH_PinkSmiley="Jiggling_Content@2x.png";
const STR_TITLE_PinkSmiley = "Content";
const STR_PARAGRAPH_PinkSmiley = "A huge library of activities for the jewish holidays, Hebrew, culture, Tora and many more";
const STR_BUTTONLABEL_PinkSmiley = "See our templates";
const STR_PINK = "pink";

 const STR_PATH_BlueWheel="Jiggling_Creator@2x.png";
const STR_TITLE_BlueWheel = "Create";
const STR_PARAGRAPH_BlueWheel = "Create your own activities, Teach your class to create their own games. The most fun way to learn something new.";
const STR_BUTTONLABEL_BlueWheel = "Try it for free";
const STR_DARKBLUE = "darkblue";

 const STR_PATH_GreenRectangle="Jiggling_Customize@2x.png";
const STR_TITLE_GreenRectangle = "Customize";
const STR_PARAGRAPH_GreenRectangle = "Easily, saving time way. Customize our templates for your needs. ";
const STR_BUTTONLABEL_GreenRectangle = "See our templates";
const STR_GREEN = "green";

 const STR_PATH_YellowSquare="Jiggling_Community@2x.png";
const STR_TITLE_YellowSquare = "Community";
const STR_PARAGRAPH_YellowSquare = "Meet X users around the world. See who plays now. Meet other teachers."  ;
const STR_BUTTONLABEL_YellowSquare = "Get inspired";
const STR_ORANGE = "orange";
 
const STR_PATH_BlueTriangle="Jiggling_Classroom@2x.png";
const STR_TITLE_BlueTriangle = "Classroom";
const STR_PARAGRAPH_BlueTriangle= "track your students journey, manage your lessons, See which activities are more successful.";
const STR_BUTTONLABEL_BlueTriangle = "Manage your class";
const STR_LIGHTBLUE = "lightblue";


const STR_BLUE = "blue";


export const JigglingParagraph = () => {
    return `

    <jiggling-section>

    <title-section titlecolor="${STR_PURPLE}" title="${STR_TITLE}" slot="title"></title-section>
    
    <div slot="icon-title-paragraph">
        ${IconWTitleWParagraph({path:STR_PATH_PinkSmiley,title:STR_TITLE_PinkSmiley, paragraph:STR_PARAGRAPH_PinkSmiley, buttonlabel:STR_BUTTONLABEL_PinkSmiley, color:STR_PINK,colorButton:STR_BLUE})}
    </div>
    <div slot="icon-title-paragraph">
    ${IconWTitleWParagraph({path:STR_PATH_BlueWheel,title:STR_TITLE_BlueWheel, paragraph:STR_PARAGRAPH_BlueWheel, buttonlabel:STR_BUTTONLABEL_BlueWheel, color:STR_DARKBLUE,colorButton:STR_BLUE})}
</div>
<div slot="icon-title-paragraph">
${IconWTitleWParagraph({path:STR_PATH_GreenRectangle,title:STR_TITLE_GreenRectangle, paragraph:STR_PARAGRAPH_GreenRectangle, buttonlabel:STR_BUTTONLABEL_GreenRectangle, color:STR_GREEN,colorButton:STR_BLUE})}
</div>
<div slot="icon-title-paragraph">
${IconWTitleWParagraph({path:STR_PATH_YellowSquare,title:STR_TITLE_YellowSquare, paragraph:STR_PARAGRAPH_YellowSquare, buttonlabel:STR_BUTTONLABEL_YellowSquare, color:STR_ORANGE,colorButton:STR_BLUE})}
</div>
<div slot="icon-title-paragraph">
${IconWTitleWParagraph({path:STR_PATH_BlueTriangle,title:STR_TITLE_BlueTriangle, paragraph:STR_PARAGRAPH_BlueTriangle, buttonlabel:STR_BUTTONLABEL_BlueTriangle, color: STR_LIGHTBLUE,colorButton:STR_BLUE})}
</div>
</jiggling-section>
    `
}