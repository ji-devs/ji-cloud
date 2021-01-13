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
   colorButton:string,
   colorTitle:string;
 
  }
  
  const DEFAULT_ARGS:ParagraphArgs = {
    path:"PinkSmiley.jpg",
    title:"Content",
    paragraph: "A huge library of activities for the jewish holidays, Hebrew, culture, Tora and many more",
    buttonlabel:"See our templates",
    colorButton: "blue",
    colorTitle:"#fea559",
 

  }




export const IconWTitleWParagraph = (props?:ParagraphArgs) => {

    const {path, title, paragraph,buttonlabel, colorTitle,colorButton} = props || DEFAULT_ARGS;
    return `
        <icon-wtitle-wparagraph path="${path}" title="${title}" paragraph="${paragraph}" color="${colorTitle}" >
            ${PlainTextButton({label:buttonlabel, color: colorButton, bold:false, size:" ", italic:false})} 
        </icon-wtitle-wparagraph>
    
    `
}