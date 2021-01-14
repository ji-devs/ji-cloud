import "@elements/column-details";
import "@elements/column-list";
import "@elements/buttons/icon-button";

import {IconButton} from "~/components/icon-button";

 

export default {
    title: 'Homepage Paragraph',
  }


  const STR_TITLE="Who we are"
  const STR_LINE1="  Jewish Interactive is a registered 501(c)(3)  "
  const STR_LINE2="in the US with tax ID 46-1331618  "
  const STR_LINE3="The Jewish interactive Educational Trust is a  "
  const STR_LINE4="  Section 18A (1)(a) in South Africa  "
  const STR_LINE5="   (Registration IT36/2012) (PBO 930 038 343) "
  const STR_LINE6=" Jewish Interactive is a registered charity  "
  const STR_LINE7="in the UK (Charity Number 1151408)  "
 






export const footerwhoweare= () => {

    return `
    <column-details head_title="${STR_TITLE}">
    <column-list text_line="${STR_LINE1}" ></column-list>
    <column-list text_line="${STR_LINE2}" ></column-list><br>
    <column-list text_line="${STR_LINE3}" ></column-list>
    <column-list text_line="${STR_LINE4}" ></column-list>
    <column-list text_line="${STR_LINE5}" ></column-list><br>
    <column-list text_line="${STR_LINE6}" ></column-list>
    <column-list text_line="${STR_LINE7}" ></column-list>
<icon-button size="large", label="Donate", color="blue", fontweight="bold"> </icon-button>

     </column-details>
   
    
    `
}











