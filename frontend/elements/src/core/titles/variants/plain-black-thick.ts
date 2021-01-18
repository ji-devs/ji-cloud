import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('plain-black-thick')
export class _ extends LitElement {
  static get styles() {
    return [css`
    p{
        color: #4a4a4a;
        margin-top:0;
       margin-bottom:0;
       font-weight:600;
       font-size:22px;
        

    }


   
    `];
  }

  @property()
  title:string = ""; 
 

  render() {

    const {title} = this;

    return html`
    <p>${title}</p>
  `;
  }
}