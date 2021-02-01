import { MEDIA_UI } from '@utils/path';
import "@elements/core/lists/column-list";
import "@elements/entry/home/social-networks";
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('contactus-section')
export class _ extends LitElement {
  static get styles() {
    return [css`
    h3{
      font-family: Poppins;
      font-size: 20px;
      font-weight: 800;   
      color:#ffffff;
    
    }
   main{
     
   }
   
   ul{
    list-style-type: none;
    margin:0;
    padding:0;
   }

 

  .socialnetworks{
     display:block;
     margin-top:30px;
  }

  .list{
    display:block;
    margin-top:8px;
 }
  

    `];
  }



  render() {

 
    const STR_TITLE="Contact us";
    const STR_LINE1="info@jewishinteractive.org";
    const STR_LINE2="Ji United States";
    const STR_LINE3="Tel: +1 (703) 517-5182";
    const STR_LINE4="Ji United Kingdom";
    const STR_LINE5="Tel: +44 (0)79 6641 4417";
    const STR_LINE6="Ji South Africa";
    const STR_LINE7="Tel: +27 (79) 886 5326";
    const STR_LINE8="Ji Israel";
    const STR_LINE9="Tel: +972 (0) 54-597 9555"  ;
   
 
    return html`
    <main>
    <h3>${STR_TITLE} </h3>
    <ul>
    <column-list text_line="${STR_LINE1}" color="white" class="list"></column-list>
    <column-list text_line="${STR_LINE2}" color="white" class="list" bold=true ></column-list>
    <column-list text_line="${STR_LINE3}" color="white" class="list"></column-list>
    <column-list text_line="${STR_LINE4}" color="white" bold=true class="list"></column-list>
    <column-list text_line="${STR_LINE5}" color="white" slot="list"></column-list>
    <column-list text_line="${STR_LINE6}" color="white" bold=true class="list"></column-list>
    <column-list text_line="${STR_LINE7}" color="white" class="list"></column-list>
    <column-list text_line="${STR_LINE8}" color="white" class="list" bold=true></column-list>
    <column-list text_line="${STR_LINE9}" color="white" class="list" ></column-list>  
    
    </slot>

    </ul>
    <social-networks class="socialnetworks" path_instagram="Icn_Instagram.png" path_facebook="icn_facebook.png"   path_youtube="Icn_Youtube.png" path_linkedin="Icn_Linkdin.png"> </social-networks>
    </main>
  `;
  }
}