import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('column-details')
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
   }
    `];
  }


  @property()
  head_title:string = ""; 


 

  render() {

    const {head_title} = this;

    return html`
    <main>
    <h3>${head_title}</h3>
    <ul>
    <slot></slot>

    </ul>

    </main>
  `;
  }
}