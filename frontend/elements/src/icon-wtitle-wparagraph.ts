import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('icon-wtitle-wparagraph')
export class _ extends LitElement {
  static get styles() {
    return [css`
  
  div{
      height:100px;
      width:100px;
      background-color:red;
  }
  p{
      font-size:30px;
  }
    `];
  }

  @property()
  label:string = ""; 

  @property()
  text:string = ""; 

  render() {

    const {label, text} = this;

    return html`
        <div>
            <p>${label} ${text}</p>
        </div>
  `;
  }
}