import { MEDIA_UI } from '@utils/path';
import "@elements/entry/home/TOSORT/column-list";
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

    const {} = this;

    return html`
    <main>
    <h3> Contact us </h3>
    <ul>
    <column-list text_line="info@jewishinteractive.org" color="white" class="list"></column-list>
    <column-list text_line="Ji United States" color="white" class="list" bold=true ></column-list>
    <column-list text_line="Tel: +1 (703) 517-5182" color="white" class="list"></column-list>
    <column-list text_line="Ji United Kingdom" color="white" bold=true class="list"></column-list>
    <column-list text_line="Tel: +44 (0)79 6641 4417" color="white" class="list"></column-list>
    <column-list text_line="Ji South Africa" color="white" bold=true class="list"></column-list>
    <column-list text_line="Tel: +27 (79) 886 5326" color="white" class="list"></column-list>
    <column-list text_line="Ji Israel" color="white" class="list" bold=true></column-list>
    <column-list text_line="Tel: +972 (0) 54-597 9555" color="white" class="list" ></column-list>    
    
    </slot>

    </ul>
    <social-networks class="socialnetworks" path_instagram"Icn_Instagram.png" path_facebook="icn_facebook.png"   path_youtube"Icn_Youtube.png" path_linkedin"Icn_Linkdin.png"> </social-networks>
    </main>
  `;
  }
}