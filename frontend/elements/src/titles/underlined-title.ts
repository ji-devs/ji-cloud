import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('underlined-title')
export class _ extends LitElement {
  static get styles() {
    return [css`
    .wrapper{
        border-bottom: solid 1px #e5e7ef;
        width:100%
    }
        h1{
        font-size: 24px;
        font-weight: 300;
        margin-top:0;
        
    }
   
    `];
  }

  @property()
  title:string = ""; 

  render() {

    const {title} = this;

    return html`
    <div class="wrapper">
       <h1>${title}</h1>
    </div>
  `;
  }
}