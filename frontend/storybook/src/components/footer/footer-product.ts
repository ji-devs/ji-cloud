import "@elements/column-details";
import "@elements/column-list";
 

export default {
    title: 'Homepage Paragraph',
  }


  const STR_TITLE="Product"
  const STR_LINE1="Manage"
  const STR_LINE2="Classroom"
  const STR_LINE3="Create activities"
  const STR_LINE4="Go pro"
  



export const footerproduct= () => {

    return `
    <column-details head_title="${STR_TITLE}">
    <column-list text_line="${STR_LINE1}" slot="list" ></column-list>
    <column-list text_line="${STR_LINE2}" slot="list" ></column-list>
    <column-list text_line="${STR_LINE3}" slot="list" ></column-list>
    <column-list text_line="${STR_LINE4}" slot="list" ></column-list>
     
     </column-details>
   
    
    `
}











