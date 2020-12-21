import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('checkbox-list')
export class _ extends LitElement {
  static get styles() {
    return [css`
  li{
      margin-bottom:12px;
  }
    `];
  }

  @property()
  path:string = ""; 

  render() {

    const {path} = this;

    return html`
    <ul>
        <li>
            <slot name="one"></slot>
        </li>
    </ul>
  `;
  }
}