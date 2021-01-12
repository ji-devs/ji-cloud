import "@elements/icon-wtitle-wparagraph";
import { PlainTextButton } from "./plain-text-button";
import{IconWTitleWParagraph} from "../components/icon-wtitle-wparagraph";
export default {
  title: 'Homepage Paragraph',
}
// <style>

// </style>

const STR_PATH ="PinkSmiley.jpg";
const STR_TITLE = "content";
const STR_PARAGRAPH = "A huge library of activities for the jewish holidays, Hebrew, culture, Tora and many more";
const STR_BUTTONLABEL="See our templates";
const STR_BUTTONCOLOR="blue";

export const why_ji = () => {
  return `
  // <h1 style="  font-size: 64px;font-weight: 900;  line-height: 1.48;color:#5662a3 ;text-align: center; ">Why Ji?</h1>
    <div style="display: inline-block;">${IconWTitleWParagraph(STR_PATH,STR_TITLE,STR_PARAGRAPH,STR_BUTTONLABEL)};</div>    
    <div style="display: inline-block;">${IconWTitleWParagraph(STR_PATH,"create","Create your own activities, Teach your class to create their own games. The most fun way to learn something new.","Try it for free")};</div>    
    <div style="display: inline-block;">${IconWTitleWParagraph(STR_PATH,"customize","Easily, saving time way. Customize our templates for your needs.","See our templates")};</div>    
    <div style="display: inline-block;">${IconWTitleWParagraph(STR_PATH,"community","Meet X users around the world. See who plays now. Meet other teachers.","Get inspired")};</div>    
    <div style="display: inline-block;">${IconWTitleWParagraph(STR_PATH,"classroom","track your students journey, manage your lessons, See which activities are more successful.","Manage your class")};</div>    


    `;
}

