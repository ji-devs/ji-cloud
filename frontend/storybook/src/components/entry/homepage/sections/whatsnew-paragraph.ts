import "@elements/entry/home/TOSORT/icon-wparagraph";
import "@elements/entry/home/sections/whatsnew-section";
import "@elements/core/titles/variants/title-section";
import "@elements/core/titles/variants/subtitle";
import "@elements/entry/home/TOSORT/list-type";
import "@elements/core/buttons/rectangle";
import "@elements/core/dividers/circle-div";
import "@elements/entry/home/TOSORT/content-wimg";


  
export default {
  title: 'Entry/ Homepage / Section',
}




const STR_TITLE = "What’s new?";
const STR_SUBTITLE="HOP TV - New Hebrew Series";



 const STR_PATHGIRL = "girl@2x.jpg";
const STR_LINE1="Learning Hebrew with HOP Channel, Learning Hebrew with ";
const STR_LINE2="HOP Channel, Learning Hebrew with HOP Channel, Learning ";
const STR_LINE3="Hebrew with HOP Channel Learning Hebrew with HOP ";
 

const STR_PLAY="Play Series";


export const whatsnewParagraph = () => {
    return `

    <whatsnew-section>
    <title-section titlecolor="white" title="${STR_TITLE}"  slot="title"></title-section>

    <content-wimg slot="contentpage" pathimg="${STR_PATHGIRL}">
    <sub-title size="large" slot="subtitle" title="${STR_SUBTITLE}" color="pink"></sub-title>


     <column-list slot="line" text_line="${STR_LINE1}" size="medium"></column-list>
     <column-list slot="line" text_line="${STR_LINE2}" size="medium"></column-list>
     <column-list slot="line" text_line="${STR_LINE3}" size="medium"></column-list>

<button-rect slot="button" size="large" color="pink"  bold="true" imglefthidden="true" imgrighthidden="true"> ${STR_PLAY}</button-rect>
     </content-wimg>

      <circle-div slot="point" color="yellow" size="medium"></circle-div>
     <circle-div slot="point" color="white" size="medium"></circle-div>
     <circle-div slot="point" color="white" size="medium"></circle-div>
     
     </whatsnew-section>
    `
}