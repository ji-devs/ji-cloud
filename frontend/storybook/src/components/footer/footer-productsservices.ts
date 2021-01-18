import "@elements/column-details";
import "@elements/column-list";
export default {
    title: 'Homepage Paragraph',
  }
  const STR_TITLE="Products & Services";
  const STR_LINE1="Teachers";
  const STR_LINE2="Parents";
  const STR_LINE3="JI Bites";
  const STR_LINE4="JI Prime";
  const STR_LINE5="JI Tap";
  const STR_LINE6="JI Studio";
  const STR_LINE7="The JI Collection";
  const STR_LINE8="J-Stream";
  const STR_LINE9="JI Blog";
  const STR_LINE10="Jobs";
export const footerproductsservices= () => {
    return `
    <column-details head_title="${STR_TITLE}">
    <column-list text_line="${STR_LINE1}" slot="list" ></column-list>
    <column-list text_line="${STR_LINE2}" slot="list" ></column-list>
    <column-list text_line="${STR_LINE3}" slot="list" ></column-list>
    <column-list text_line="${STR_LINE4}" slot="list" ></column-list>
    <column-list text_line="${STR_LINE5}" slot="list" ></column-list>
    <column-list text_line="${STR_LINE6}" slot="list" ></column-list>
    <column-list text_line="${STR_LINE7}" slot="list" ></column-list>
    <column-list text_line="${STR_LINE8}" slot="list" ></column-list>
    <column-list text_line="${STR_LINE9}" slot="list" ></column-list>
    <column-list text_line="${STR_LINE10}" slot="list" ></column-list>
     </column-details>
    `
}