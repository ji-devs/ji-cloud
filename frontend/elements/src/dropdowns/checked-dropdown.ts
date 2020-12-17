import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('checkeddropdown')
export class _ extends LitElement {
  static get styles() {
    return [css`
    .dropdown-wrapper{
        postion:relative;
    }
    `];
  }

  @property()
  label: string = "";

  render() {

    const {label} = this;

    return html`
     <main class="">
      <div class="dropdown-wrapper">
      <span><p>${label}</p><img src="${MEDIA_UI}/icons/icn-chevron-idle.svg"/></span>
      </div>
    </main>
  `;
  }
}