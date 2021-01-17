import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('column-list')
export class _ extends LitElement {
    static get styles() {
    return [css`
    li{
      font-family: Poppins;
      font-size: 14px;
      font-weight: 200;
      color:#ffffff;
      list-style-type: none;

    

     }
     .bold {
       font-weight:600;
     }
    `];
  }


  @property()
  text_line:string = ""; 

  @property({type:Boolean})
  bold:boolean = false; 
 

  render() {
    const {text_line, bold} = this;

    return html`
     <li class=${bold ? "bold" : ''}>${text_line}</li>
  `;
  }
}