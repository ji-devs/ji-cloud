import "@elements/entry/home/TOSORT/column-details";
import "@elements/entry/home/TOSORT/column-list";
export default {
    title: 'Homepage Paragraph',
  }
  const STR_TITLE="Product"; 
  const STR_LINE1="Manage";
  const STR_LINE2="Classroom";
  const STR_LINE3="Create activities";
  const STR_LINE4="Go pro";
  const STR_WHITE ="white";

export const footerproduct= () => {
    return `
    <column-details head_title="${STR_TITLE}">
    <column-list text_line="${STR_LINE1}" color="${STR_WHITE}" slot="list" ></column-list>
    <column-list text_line="${STR_LINE2}" color="${STR_WHITE}" slot="list" ></column-list>
    <column-list text_line="${STR_LINE3}" color="${STR_WHITE}" slot="list" ></column-list>
    <column-list text_line="${STR_LINE4}" color="${STR_WHITE}" slot="list" ></column-list>
     </column-details>
    `
}