import "@elements/icon-wtitle-wparagraph";
import { PlainTextButton } from "~/components/plain-text-button";
export default {
  title: 'Homepage Paragraph',
}

interface ParagraphArgs {
   path:string,
   title:string,
   paragraph: string,
   buttonlabel: string,
   color:string,
  
  }
  
  const DEFAULT_ARGS:ParagraphArgs = {
    path:"PinkSmiley.jpg",
    title:"Content",
    paragraph: "A huge library of activities for the jewish holidays, Hebrew, culture, Tora and many more",
    buttonlabel:"See our templates",
    color: "blue",
  }


const STR_BUTTONLABEL = "";
const STR_BLUE = "blue";


export const IconWTitleWParagraph = (props?:ParagraphArgs) => {

    const {path, title, paragraph,buttonlabel, color} = props || DEFAULT_ARGS;
    return `
        <icon-wtitle-wparagraph path="${path}" title="${title}" paragraph="${paragraph}">
            ${PlainTextButton({label:buttonlabel, color: color, bold:false, size:"", italic:false})} 
        </icon-wtitle-wparagraph>
    
    `
}