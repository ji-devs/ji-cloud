import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
import "@elements/entry/home/TOSORT/column-list";
@customElement('footer-help')
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

    return html`
    <main>
    <h3>Help</h3>
    <ul>
    <column-list text_line="Support & FAQ" color="white" class="list" ></column-list>
    <column-list text_line="Quick tour" color="white" class="list" ></column-list>
    <column-list text_line="JI Tutorials" color="white" class="list" ></column-list>
    <column-list text_line="Online webinars" color="white" class="list" ></column-list>
    <column-list text_line="Accessibility" color="white" class="list" ></column-list>

    </ul>
    
    </main>
  `;
  }
}