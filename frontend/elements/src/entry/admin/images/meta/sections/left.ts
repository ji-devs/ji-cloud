import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('section-left')
export class _ extends LitElement {
  static get styles() {
    return [css`
 
    .wrapper{
        width:288px;
    }
    ::slotted([slot=image-actions]){
     
    }
    ::slotted([slot=divider]){
      margin: 0 16px;
    }
    ::slotted([slot=checkbox]){
      margin-bottom: 34px;
    }
    .image-actions-wrapper{
      display:flex;
      align-items:center;
      justify-content:flex-end;
    }
    `];
  }

  render() {

    const {} = this;

    return html`
    <div class="wrapper">
        <slot name="image"></slot>
        <div class="image-actions-wrapper">
          <slot name="image-actions"></slot>
        </div>
        <slot name="divider"></slot>
        <slot name="checkbox"></slot>
        <slot name="description"></slot>
    </div>
  `;
  }
}