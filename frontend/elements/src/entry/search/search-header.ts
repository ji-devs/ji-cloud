import { LitElement, html, css, customElement, property } from 'lit-element';
import "@elements/entry/home/sections/search-bar";

@customElement('search-header')
export class _ extends LitElement {
  static get styles() {
    return [css`
    main{
        width: 1920px;
        height: 216px;
        background-color:#6ca1fc;
        position:relative;
    }
    .magnifying{
      position:absolute;
      bottom:0;
      right:418px;
    }
    ::slotted([slot="header"]){
      display:block;
      margin-left:96px;
      padding-top:84px;
    }
    .helper{
      position:absolute;
      right:36px;
      top: 36px;
    }
    `];
  }

  render() {

    const {} = this;

    return html`    
    <main>
        <slot name="header"></slot>
        <img-ui class="magnifying" path="searchmagnifying.svg"></img-ui>
        <img-ui class="helper" path="rectangle-2382.png"></img-ui>
    
    </main>

  `;
  }
}