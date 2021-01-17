import "@elements/column-details";
import "@elements/column-list";
 

export default {
    title: 'Homepage Paragraph',
  }


  const STR_TITLE="JIGs"
  const STR_LINE1="Hebrew JIGs"
  const STR_LINE2="Jewish holidays JIGs"
  const STR_LINE3="Torah JIGs"
  const STR_LINE4="J-Stream"
  const STR_LINE5="Israel"
  const STR_LINE6="Songs"
  
  




export const footerjigs= () => {

    return `
    <column-details head_title="${STR_TITLE}">
    <column-list text_line="${STR_LINE1}" slot="list" ></column-list>
    <column-list text_line="${STR_LINE2}" slot="list" ></column-list>
    <column-list text_line="${STR_LINE3}" slot="list" ></column-list>
    <column-list text_line="${STR_LINE4}" slot="list" ></column-list>
    <column-list text_line="${STR_LINE5}" slot="list" ></column-list>
    <column-list text_line="${STR_LINE6}" slot="list" ></column-list>
     </column-details>
   
    
    `
}











