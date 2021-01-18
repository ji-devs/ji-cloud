import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('lineoflist')
export class _ extends LitElement {
    static get styles() {
    return [css`
    li{
      font-family: Poppins;
      font-size: 20px;
     }
 
    `];
  }


  @property()
  text_line:string = ""; 

   

  render() {
    const {text_line} = this;

    return html`
     <li>${text_line}</li>
  `;
  }
}