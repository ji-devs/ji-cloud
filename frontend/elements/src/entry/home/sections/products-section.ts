import { MEDIA_UI } from '@utils/path';
import "@elements/entry/home/TOSORT/column-list";
import "@elements/entry/home/social-networks";
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('products-section')
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


 
 

  render() {

    const {} = this;

    return html`
    <main>
    <h3>Product </h3>
    <ul>
    <column-list text_line="Manage" color="white" slot="list" ></column-list>
    <column-list text_line="Classroom" color="white" slot="list" ></column-list>
    <column-list text_line="Create activities" color="white" slot="list" ></column-list>
    <column-list text_line="Go pro" color="white" slot="list" ></column-list>
    </ul>
    </main>
  `;
  }
}