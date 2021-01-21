import "@elements/entry/home/TOSORT/column-details";
import "@elements/entry/home/TOSORT/column-list";
import "@elements/core/buttons/rectangle";


export default {
    title: 'Homepage Paragraph',
  }
  const STR_TITLE="Who we are";
  const STR_LINE1="  Jewish Interactive is a registered 501(c)(3)  ";
  const STR_LINE2="in the US with tax ID 46-1331618  ";
  const STR_LINE3="The Jewish interactive Educational Trust is a  ";
  const STR_LINE4="  Section 18A (1)(a) in South Africa  ";
  const STR_LINE5="   (Registration IT36/2012) (PBO 930 038 343) ";
  const STR_LINE6=" Jewish Interactive is a registered charity  ";
  const STR_LINE7="in the UK (Charity Number 1151408)  ";
  const STR_WHITE ="white";
  const STR_DARKBLUE="darkblue";
  const STR_LINE10="                        "; 
  const STR_DONATE="Donate";

export const footerwhoweare= () => {
    return `
    <column-details head_title="${STR_TITLE}">
    <column-list text_line="${STR_LINE1}" color="${STR_WHITE}" slot="list" ></column-list>
    <column-list text_line="${STR_LINE2}" color="${STR_WHITE}" slot="list"></column-list><br>
    <column-list text_line="${STR_LINE10}" color="${STR_DARKBLUE}" slot="list"></column-list>

    <column-list text_line="${STR_LINE3}" color="${STR_WHITE}" slot="list"></column-list>
    <column-list text_line="${STR_LINE4}" color="${STR_WHITE}" slot="list"></column-list>
    <column-list text_line="${STR_LINE5}" color="${STR_WHITE}" slot="list"></column-list><br>
    <column-list text_line="${STR_LINE10}" color="${STR_DARKBLUE}" slot="list"></column-list>

    <column-list text_line="${STR_LINE6}" color="${STR_WHITE}" slot="list"></column-list>
    <column-list text_line="${STR_LINE7}" color="${STR_WHITE}" slot="list"></column-list>
<button-rect slot="Donate" size="medium"  color="blue"  bold="true" imglefthidden="true" imgrighthidden="true">${STR_DONATE} </button-rect>

     </column-details>
    `
}