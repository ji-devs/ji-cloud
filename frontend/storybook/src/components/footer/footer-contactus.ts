import "@elements/column-details";
import "@elements/column-list";
 

export default {
    title: 'Homepage Paragraph',
  }


  const STR_TITLE="Contact us"
  const STR_LINE1="info@jewishinteractive.org"
  const STR_LINE2="Ji United States"
  const STR_LINE3=" Tel: +1 (703) 517-5182"
  const STR_LINE4="  Ji United Kingdom"
  const STR_LINE5="   Tel: +44 (0)79 6641 4417  "
  const STR_LINE6="   Ji South Africa   "
  const STR_LINE7="  Tel: +27 (79) 886 5326  "
  const STR_LINE8="  Ji Israel  "
  const STR_LINE9="  Tel: +972 (0) 54-597 9555"  

  



export const footercontactus= () => {

    return `
    <column-details head_title="${STR_TITLE}">
    <column-list text_line="${STR_LINE1}" slot="list"></column-list>
    <column-list text_line="${STR_LINE2}" slot="list" bold=true ></column-list>
    <column-list text_line="${STR_LINE3}" slot="list"></column-list>
    <column-list text_line="${STR_LINE4}" bold=true slot="list"></column-list>
    <column-list text_line="${STR_LINE5}" slot="list"></column-list>
    <column-list text_line="${STR_LINE6}" bold=true slot="list"></column-list>
    <column-list text_line="${STR_LINE7}" slot="list"></column-list>
    <column-list text_line="${STR_LINE8}" slot="list" bold=true></column-list>
    <column-list text_line="${STR_LINE9}" slot="list" ></column-list>
 

     </column-details>
   
    
    `
}











