import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';


@customElement('tree-dropdown-child')
export class _ extends LitElement {

  static get styles() {
    return [css`
    li{
        position:relative;
        display:flex;
        content-items:center;

    }
    .open img-ui{
        transform: rotate(90deg);
    }
    `];
  }

@property()
label: string = "";

@property()
path: string = "";

@property()
open: boolean = false;

  render() {

    const {label, path, open} = this;

    return html`
  
  <li class="titleoptions open">
    <div data-id="label"></div>
    <div class="absolute border-l border-b border-jiblueLight w-6 h-6 spacer">
    </div>
    <div class="flex mr-2 pl-2 relative">
      <img-ui data-id="arrow" class="px-1" path="/icon-chevron-categories-24-px.svg" alt="">
      <p class="font-normal">Hello</p>
    </div>

  </li>

  `;
  }
}