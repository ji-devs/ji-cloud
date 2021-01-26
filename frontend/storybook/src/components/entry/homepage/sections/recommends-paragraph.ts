import "@elements/entry/home/TOSORT/icon-wparagraph";
import "@elements/entry/home/sections/recommend";
 
export default {
  title: 'Homepage',
}


const STR_TITLE_HEBREW = "Hebrew";
const STR_PARAGRAPH_HEBREW = "1,850 JIGs";
 const STR_DARKGRAY = "#272727";

const STR_TITLE_CHANUKAH = "Chanukah";
const STR_PARAGRAPH_CHANUKAH = "355 JIGs";

const STR_TITLE_HEBREWAROUNDTHEWORLD = "Hebrew Around the World";
const STR_PARAGRAPH_HEBREWAROUNDTHEWORLD = "World wide program";
  
const STR_TITLE_KIDSINSPACE = "Kids in Space";
const STR_PARAGRAPH_KIDSINSPACE = "J-Steam"  ;
  
const STR_TITLE_HOP = "HOP";
const STR_PARAGRAPH_HOP= "Hebrew Series";
 

 


export const RecommendsParagraph = () => {
    return `

    <recommends-section>
      
    <div slot="icon-title-paragraph">
    <icon-wparagraph path="Image_Hebrew.png" title="${STR_TITLE_HEBREW}" paragraph="${STR_PARAGRAPH_HEBREW}" color="${STR_DARKGRAY}" >
    </icon-wparagraph>

     </div>
    <div slot="icon-title-paragraph">
    <icon-wparagraph path="Image_Chanukah.png" title="${STR_TITLE_CHANUKAH}" paragraph="${STR_PARAGRAPH_CHANUKAH}" color="${STR_DARKGRAY}" >
    </icon-wparagraph>

 </div>
<div slot="icon-title-paragraph">
<icon-wparagraph path="Image_AroundWorld.png" title="${STR_TITLE_HEBREWAROUNDTHEWORLD}" paragraph="${STR_PARAGRAPH_HEBREWAROUNDTHEWORLD}" color="${STR_DARKGRAY}" >
 </icon-wparagraph>

 </div>
<div slot="icon-title-paragraph">
<icon-wparagraph path="Image_JStream.png" title="${STR_TITLE_KIDSINSPACE}" paragraph="${STR_PARAGRAPH_KIDSINSPACE}" color="${STR_DARKGRAY}" >
 </icon-wparagraph>
 </div>
<div slot="icon-title-paragraph">
<icon-wparagraph path="Image_hop.png" title="${STR_TITLE_HOP}" paragraph="${STR_PARAGRAPH_HOP}" color="${STR_DARKGRAY}" >
 </icon-wparagraph>
 </div>
</recommends-section>
    `
}