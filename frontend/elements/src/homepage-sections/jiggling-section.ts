import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('jiggling-section')
export class _ extends LitElement {
  static get styles() {
    return [css`
    .inside-wrapper{
        display:flex;
    }
   
   
    `];
  }



  render() {

    const {} = this;

    return html`
    <main>
    <slot name="title"></slot>
    <div class="inside-wrapper">
        <slot name="icon-title-paragraph"></slot>
    </div>
    </main>
  `;
  }
}