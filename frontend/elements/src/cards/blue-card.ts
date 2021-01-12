import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';

@customElement('blue-card')
export class _ extends LitElement {

  static get styles() {
    return [css`
    div{
        height: 696px;
        border-radius: 10px;
        background-color: #edf2ff;
        overflow:auto;
    }
    `];
  }

@property()
label: string = "";

  render() {
    const {label} = this;
    return html`
<div>
    <slot name="content"></slot>
</div>
  `;
  }
}