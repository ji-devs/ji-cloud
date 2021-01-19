import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('list-height')
export class _ extends LitElement {
  static get styles() {
    return [css`
    ul{
        padding:0;
        margin:0;
        max-height:400px;
        overflow:auto
    }
    `];
  }

  render() {
    return html`
    <ul>
      <slot></slot>
    </ul>
  `;
  }
}