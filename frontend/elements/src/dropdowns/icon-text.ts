import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('icontext-dropdown')
export class _ extends LitElement {
  static get styles() {
    return [css`
    main{
        padding: 14px 8px 14px 0;
        border-radius: 8px;
        -webkit-backdrop-filter: blur(30px);
        backdrop-filter: blur(30px);
        box-shadow: 0 3px 16px 0 rgba(0, 0, 0, 0.2);
        background-color: #ffffff;
        width:244px;
        position:absolute;
        
    }
    .open {
        display:block;
    }
    .closed {
        display:none;
    }
    
    `];
  }

  @property()
  label: string = "";

  @property()
  open: boolean = false;

  render() {

    const {open} = this;

    return html`
     <main class="${open ? "open" : 'closed'}">
      <div class="dropdown-wrapper">
        <slot></slot>
      </div>
    </main>
  `;
  }