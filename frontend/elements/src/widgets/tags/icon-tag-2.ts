import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('icon-wtext')
export class _ extends LitElement {
  static get styles() {
    return [css`
    div{
        display:flex;
        align-items:center;
    }
   img-ui{
       margin-right:12px;
   }
   div:hover p{
       color:#5590fc
   }
   
    `];
  }

  @property()
  icon: string = "";

  @property()
  text: string = "";


  render() {

    const {icon,text} = this;

    return html`
    
    <div>
        <img-ui path="${icon}"></img-ui>
        <p>${text}</p>
    </div>
  `;
  }
}