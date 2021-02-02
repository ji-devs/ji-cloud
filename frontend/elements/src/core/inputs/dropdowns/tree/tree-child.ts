import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/widgets/tags/ellipses";

import { nothing } from "lit-html";
export type Mode = "checkbox" | "inputText";
export type Page = "category" | "image";
@customElement("dropdown-tree-child")
export class _ extends LitElement {
  static get styles() {
    return [
      css`
      li{
          position:relative;
          display:flex;
          content-items:center;

      }
      .open img-ui{
          transform: rotate(90deg);
      }
      ul.closed {
        display: none;
      }
      .inside {
        position:absolute;
        border:solid 1px #c4dbff;
        border-right:none; 
        border-top:none;
        width:26px;
        height:24px;
        left:-28px;
        top: -10px


      }
      .icon-wrapper{
        position:relative;
        
        
      }
      .titleoptions{
        margin-top:8px;
        position: relative;
        top: 6px;
       
        
      }
      .ellipses-wrapper{
        display:flex;
        position:relative;
       
      }
      .ellipses{
        display:none;
        position:absolute;
        left:50px;
      }
      .ellipses-wrapper:hover .ellipses{
        display:flex;
      }
      ::slotted(*){
        display:flex;
      }
      .noellipses{
        display:none;
      }
      
    `,
    ];
  }

  @property()
  label: string = "";

  @property({ type: Boolean })
  open: boolean = false;

  @property()
  content: Mode = "inputText";

  @property()
  page: Page = "image";

  render() {
    const { label, open, content, page } = this;
    const inside = content === "checkbox" ? html`
    <input type="checkbox"/>
    <div class="inside"></div>`
    : content === "inputText" ? html`
    <img-ui path="icon-chevron-categories-24-px.svg" alt=""></img-ui>
    <div class="inside"></div>
    `
    : nothing;

    const ellipses = page === "category" ? html`<tag-ellipses class="ellipses"></tag-ellipses>`
    : nothing;

    return html`
      <li class="titleoptions open">
      <div class="ellipses-wrapper">
        <div class="icon-wrapper">
        ${inside}
          
        </div>
        <div>${label}</div>
        ${ellipses}
        </div>
        
        <ul class="${open ? "open" : "closed"}">
          <slot></slot>
        </ul>
      </li>

        
    `;
  }
}
