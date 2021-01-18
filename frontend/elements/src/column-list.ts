import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('column-list')
export class _ extends LitElement {
    static get styles() {
    return [css`
    .normal li{
      font-family: Poppins;
      font-size: 14px;
      font-weight: 200;
      color:#ffffff;
      list-style-type: none;

    

     }
     .bold {
       font-weight:600;
       color:#ffffff;

     }

     .medium{
      font-size: 20px;
      font-weight: normal;
      list-style-type: none;

     }
    `];
  }


  @property()
  text_line:string = ""; 

  @property({type:Boolean})
  bold:boolean = false; 

  @property()
  size:string = ""; 
 

  render() {
    const {text_line, bold,size} = this;

    return html`
     <li class="${bold ? "bold" : ''} ${ size ? "medium":"normal"}" >${text_line}</li>
    
  `;
  }
}