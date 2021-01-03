import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('sub-title')
export class _ extends LitElement {
  static get styles() {
    return [css`
    p{
      
        font-size: 20px;
        font-weight: 300;
        line-height:0.5;
        margin-bottom:32px;
    }

    
   
    `];
  }

  @property()
  title:string = ""; 
  

  render() {

    const {title, } = this;

    return html`
    <p>${title}</p>
  `;
  }
}