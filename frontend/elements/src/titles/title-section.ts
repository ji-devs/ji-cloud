import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('title-section')
export class _ extends LitElement {
  static get styles() {
    return [css`
  h1{
    font-size: 64px;
    font-weight: 900;
  

  }
  .purple{
    color: #5662a3;
  }
 
    `];
  }


  @property()
  title:string = ""; 

  @property()
  titlecolor:string = "";
  
 

  render() {
    const {title,titlecolor} = this;

    return html`
        <h1 class="${titlecolor}">${title}</h1> 
  `;
  }
}