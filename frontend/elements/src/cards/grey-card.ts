import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';

@customElement('grey-card')
export class _ extends LitElement {

  static get styles() {
    return [css`
    div{
        padding: 32px 32px 32px 32px;
        border-radius: 14px;
        background-color: #f7f7f7;
        width:inherit;
    }
    `];
  }

@property()
label: string = "";

  render() {
    const {label} = this;
    return html`
<div>
    <slot name="title"></slot>
    <slot name="content"></slot>
    <slot></slot>
</div>
  `;
  }
}