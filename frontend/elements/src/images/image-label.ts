import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('image-label')
export class _ extends LitElement {
  static get styles() {
    return [css`
    img{
        width: 288px;
        height: 216px;
        object-fit: contain;
        box-shadow: 0 3px 6px 0 rgba(0, 0, 0, 0.16);
    }
    .slot{
        display:flex;
        justify-content: space-between;
        padding:0 1px;
    }
    `];
  }

  @property()
  path:string = ""; 

  render() {

    const {path} = this;

    return html`
    <div>
    <!-- replace with img-ji on live -->
    <img-ui path=${path}/>
    <div class="slot">
        <slot name=one></slot>
        <slot name=two></slot>
    </div>
    </div>
  `;
  }
}