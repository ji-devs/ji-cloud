import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('list-hover')
export class _ extends LitElement {
  static get styles() {
    return [css`
  li{
      margin-bottom:12px;
  }
  li:hover{
      background-color: #e5e7ef;
  }
    `];
  }

 

  render() {

    const {} = this;

    return html`
    
    <ul>
        <li>
            <slot name="one"></slot>
        </li>
    </ul>
  `;
  }
}