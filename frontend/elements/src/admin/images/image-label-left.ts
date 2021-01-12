import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('imagelabel-left')
export class _ extends LitElement {
  static get styles() {
    return [css`
 
    .wrapper, ::slotted(*){
        width:288px;
    }
    ::slotted([slot=image-actions]){
      display:flex;
      justify-content: flex-end;
      width:100%;
    }
    ::slotted([slot=divider]){
      margin: 0 16px;
    }
    ::slotted([slot=checkbox]){
      margin-bottom: 34px;
    }
    `];
  }

  render() {

    const {} = this;

    return html`
    <div class="wrapper">
        <slot name="image"></slot>
        <slot name="image-actions"></slot>
        <slot name="divider"></slot>
        <slot name="checkbox"></slot>
        <slot name="description"></slot>
    </div>
  `;
  }
}