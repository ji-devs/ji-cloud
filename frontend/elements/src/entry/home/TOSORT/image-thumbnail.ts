import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('image-thumbnail')
export class _ extends LitElement {
  static get styles() {
    return [css`
  
    .img{
        width: 296px;
        height: 167px;
        border-radius: 16px;
        margin-bottom:32px;
    }
   
    `];
  }

  @property()
  path:string = ""; 
  

  render() {

    const {path} = this;

    return html`
    <img-ui class="img" path="${path}"></img-ui>
  `;
  }
}