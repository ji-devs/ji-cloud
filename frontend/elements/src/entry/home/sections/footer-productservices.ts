import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
import "@elements/entry/home/TOSORT/column-list";
@customElement('footer-productservices')
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


  .list{
    display:block;
    margin-top:8px;
 }
  

    `];
  }


  @property()
  head_title:string = ""; 


 

  render() {

    const {head_title} = this;

    const STR_TEACHERS = "Teachers";
    const STR_PARENTS = "Parents";

    return html`
    <main>
    <h3>Products & Services</h3>
    <ul>
    <column-list text_line="${STR_TEACHERS}" color="white" class="list" ></column-list>
    <column-list text_line="${STR_PARENTS}" color="white" class="list" ></column-list>
    <column-list text_line="JI Bites" color="white" class="list" ></column-list>
    <column-list text_line="JI Prime" color="white" class="list" ></column-list>
    <column-list text_line="JI Tap" color="white" class="list" ></column-list>
    <column-list text_line="JI Studio" color="white" class="list" ></column-list>
    <column-list text_line="The JI Collection" color="white" class="list" ></column-list>
    <column-list text_line="J-Stream" color="white" class="list" ></column-list>
    <column-list text_line="JI Blog" color="white" class="list" ></column-list>
    <column-list text_line="Jobs" color="white" class="list" ></column-list>

    </ul>
    
    </main>
  `;
  }
}