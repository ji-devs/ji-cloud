import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('plain-blue')
export class _ extends LitElement {
  static get styles() {
    return [css`
    p{
        color: #5590fc;
        

    }
    .bold {
      font-weight:500;
    }
    
   
    `];
  }

  @property()
  title:string = ""; 
  @property({type: Boolean})
  bold:boolean = false; 

  render() {

    const {title, bold} = this;

    return html`
    <p class="${bold}">${title}</p>
  `;
  }
}