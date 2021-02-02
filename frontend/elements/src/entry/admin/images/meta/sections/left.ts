import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('section-left')
export class _ extends LitElement {
  static get styles() {
    return [css`
 
    .left-wrapper{
        width:288px;
    }
    ::slotted([slot=image-actions]){
     height:20px;
    }
    ::slotted([slot=divider]){
      margin: 0 16px;
    }
    ::slotted([slot=checkbox]){
      margin-bottom: 34px;
      display:block;
    }
    .image-actions-wrapper{
      display:flex;
      align-items:center;
      justify-content:flex-end;
      margin-top:16px;
      border-bottom:solid 1px #e5e7ef;
      padding-bottom:16px;
      margin-bottom:16px;
    `];
  }

  render() {

    return html`
    <div class="left-wrapper">
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