import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('title-paragraph')
export class _ extends LitElement {
  static get styles() {
    return [css`
  h1{
    font-size: 64px;
    font-weight: 900;
    color:#5662a3;

  }
 
    `];
  }


  @property()
  title:string = ""; 
  
 

  render() {
    const {title} = this;

    return html`
        <h1>${title}</h1> 
  `;
  }
}