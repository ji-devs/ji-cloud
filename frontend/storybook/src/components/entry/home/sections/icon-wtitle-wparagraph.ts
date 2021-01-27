import "@elements/entry/home/TOSORT/create-leftparagraph";
import {Text} from "~/components/core/buttons/text";
 import { PlainTextButton } from "~/components/entry/home/sections/plain-text-button";
 import {Color, Size, Weight} from "@elements/core/buttons/text";

export default {
  title: 'Entry/ Home / Widget',
}

interface ParagraphArgs {
   path:string,
   title:string,
   paragraph: string,
   contents: string,
   colorButton:Color,
   color:string;
   weight:Weight;
   size:Size;
   italic:boolean;
  }
  
  const DEFAULT_ARGS:ParagraphArgs = {
    path:"PinkSmiley.jpg",
    title:"Content",
    paragraph: "A huge library of activities for the jewish holidays, Hebrew, culture, Tora and many more",
    contents:"See our templates",
    colorButton: "blue",
    color:"#fea559",
    weight:"normal",
    size:"small",
    italic:false
  }




export const IconWTitleWParagraph = (props?:ParagraphArgs) => {

    const {path, title, paragraph,contents, color,colorButton,weight,size,italic} = props || DEFAULT_ARGS;
    return `
        <icon-wtitle-wparagraph path="${path}" title="${title}" paragraph="${paragraph}" color="${color}" >
         ${Text({contents:contents, color:colorButton, weight:weight, size:size,italic:false})} 

        </icon-wtitle-wparagraph>
    
    `
}

IconWTitleWParagraph.args = DEFAULT_ARGS;
// IconWTitleWParagraph.argTypes = {
//   color: {
//       control: {
//           type: 'inline-radio',
//           options: ["white", "black", "yellow"]
//       }
//   },
//   size: {
//     control: {
//         type: 'inline-radio',
//         options: ["small", "medium"]
//     }
// }
// }