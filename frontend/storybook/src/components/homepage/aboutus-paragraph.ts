import "@elements/icon-wparagraph";
import "@elements/homepage-sections/aboutus-section";
import "@elements/titles/title-section"
import "@elements/titles/subtitle"
import "@elements/list-type"
import "@elements/buttons/rectangle-button"
import "@elements/dividers/circle-div";
import "@elements/title-sub-paragraph"


  
export default {
  title: 'Homepage Paragraph',
}


const STR_STARTTITLE = "What they say ";
const STR_ENDTTITLE = "about us ";

const STR_TITLE = "Parents";
const STR_SUBTITLE = "Sarah Nazirah, Mexico";

const STR_PINK = "pink";
const STR_SIZELARGE = "large";
const STR_LINE1="I want to tell you, because of JI, my children are ";
const STR_LINE2="learning Hebrew and English simultaneously. For my ";
const STR_LINE3="children, you are not only teaching two children, you"
const STR_LINE4="are also saving their souls. I reaffirm that they have";
const STR_LINE5="achieved educational rehabilitation, thanks to JI.";

const STR_YELLOW="yellow";
const STR_BLACK="black";

const STR_SMALL="small";
const STR_WHITE="white";
const STR_MEDIUM="medium";


export const aboutusParagraph = () => {
    return `

    
    <aboutus-section>
   
    <title-section titlecolor="${STR_WHITE}" title="${STR_STARTTITLE}" size="${STR_SIZELARGE}" slot="title"></title-section>
    <title-section titlecolor="${STR_YELLOW}" title="${STR_ENDTTITLE}" size="${STR_SIZELARGE}" slot="title"></title-section>
<title-sub-paragraph slot="title-sub-paragraph" colortitle="${STR_YELLOW}" colorsubtitle="${STR_BLACK}" sizetitle="${STR_MEDIUM}" sizesubtitle="${STR_SMALL}" title="${STR_TITLE}" subtitle="${STR_SUBTITLE}">



<column-list slot="line" text_line="${STR_LINE1}" size="medium"></column-list>
<column-list slot="line" text_line="${STR_LINE2}" size="medium"></column-list>
<column-list slot="line" text_line="${STR_LINE3}" size="medium"></column-list>
<column-list slot="line" text_line="${STR_LINE4}" size="medium"></column-list>
<column-list slot="line" text_line="${STR_LINE5}" size="medium"></column-list>

</title-sub-paragraph>
   
<circle-div color="${STR_PINK}" size="${STR_MEDIUM}"></circle-div>
<circle-div color="${STR_WHITE}" size="${STR_MEDIUM}"></circle-div>
<circle-div color="${STR_WHITE}" size="${STR_MEDIUM}"></circle-div>
<circle-div color="${STR_WHITE}" size="${STR_MEDIUM}"></circle-div>

    </aboutus-section>
    `
}