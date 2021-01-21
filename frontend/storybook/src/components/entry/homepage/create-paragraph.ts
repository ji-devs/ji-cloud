import "@elements/entry/home/TOSORT/icon-wparagraph";
import "@elements/entry/home/sections/create-section";
import "@elements/core/titles/variants/title-section"
import "@elements/core/titles/variants/subtitle";
import "@elements/entry/home/TOSORT/list-type";
import "@elements/core/buttons/rectangle";
import "@elements/core/dividers/circle-div"
import "@elements/entry/home/TOSORT/column-list"

 
  
export default {
  title: 'Homepage',
}



const STR_SUBTITLE = "Learning Through Creation";

const STR_STARTOTITLE = "Create";

const STR_ENDOTITLE = "your own JIGs";
const STR_PINK = "pink";
const STR_DARKBLUE = "darkblue";
 const STR_SIZELARGE = "large";

 const STR_PATHGIRL = "girl@2x.jpg";
const STR_LINE1="Big content library";
const STR_LINE2="Smart & friendly interface";
const STR_LINE3="Teaching through creation"
const STR_LINE4="All in one";
const STR_YELLOW="yellow";
const STR_SMALL="small";
const STR_MEDIUM="medium";
const STR_START="Start creating";


export const createParagraph = () => {
    return `

    <create-section>
    <sub-title size="${STR_MEDIUM}" slot="subtitle" title="${STR_SUBTITLE}"></sub-title>
    <title-section titlecolor="${STR_PINK}" title="${STR_STARTOTITLE}" size="${STR_SIZELARGE}" slot="title"></title-section>
    <title-section titlecolor="${STR_DARKBLUE}" title="${STR_ENDOTITLE}" size="${STR_SIZELARGE}" slot="title"></title-section>
   
 <div slot="line">
  <circle-div color="${STR_YELLOW}" size="${STR_SMALL}"></circle-div>
 <column-list text_line="${STR_LINE1}" size="medium"></column-list>
 </div>

 <div slot="line">
 <circle-div color="${STR_YELLOW}" size="${STR_SMALL}"></circle-div>
 <column-list text_line="${STR_LINE2}" size="medium"></column-list>
 </div>

 <div slot="line">

 <circle-div color="${STR_YELLOW}" size="${STR_SMALL}"></circle-div>
 <column-list text_line="${STR_LINE3}" size="medium"></column-list>
 </div>

 <div slot="line">
 <circle-div color="${STR_YELLOW}" size="${STR_SMALL}"></circle-div>
 <column-list text_line="${STR_LINE4}" size="medium"></column-list>
 </div>

 
<button-rect slot="Start-creating" size="large"  color="pink", bold="true" imglefthidden="false" imgrighthidden="true">${STR_START} </button-rect>
<img-ui path="${STR_PATHGIRL}" slot="girl"><img-ui>

    </create-section>
    `
}