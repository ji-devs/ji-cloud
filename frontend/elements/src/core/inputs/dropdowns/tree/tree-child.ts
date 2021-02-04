import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/buttons/ellipses";
import "@elements/entry/admin/category/category-dropdown";

import { nothing } from "lit-html";
export type Mode = "checkbox" | "textInput" | "textDisplay";

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
        display:block;
      }
    
      .ellipsesMenu{
        display:block;
      }
   
    
    `,
    ];
  }

  @property()
  label: string = "";

  @property({ type: Boolean })
  open: boolean = false;

  @property()
  mode: Mode = "textDisplay";

  @property({ type: Boolean })
  hasMenu: boolean = false;

  render() {
    const { label, open, mode, hasMenu } = this;
    const inside = mode === "checkbox" ? 
      html`
      <div class="icon-wrapper">
        <input type="checkbox" />
        <div class="inside"></div>
      </div>
      <div>${label}</div>
      <button-ellipses class="ellipses ${hasMenu ? 'ellipsesMenu' : ''}">
      <category-dropdown></category-dropdown>
      </button-ellipses>
`
      : mode === "textDisplay" ? 
      html`
        <div class="icon-wrapper">
          <img-ui path="icon-chevron-categories-24-px.svg" alt=""></img-ui>
          <div class="inside"></div>
        </div>
        <div>${label}</div>
        <button-ellipses class="ellipses ${hasMenu ? 'ellipsesMenu' : ''}">
          <category-dropdown></category-dropdown>
        </button-ellipses>

        `
      : mode === "textInput" ?
      html`
      <div class="icon-wrapper">
      <img-ui path="icon-chevron-categories-24-px.svg" alt=""></img-ui>
      <input type="text"/>
      <div class="inside"></div>
      </div>`
      :nothing;



    return html`
      <li class="titleoptions open">
        <div class="ellipses-wrapper">
       
            ${inside}
        </div>
      
        <ul class="${open ? " open" : "closed" }">
          <slot></slot>
        </ul>
      </li>

        
    `;
  }
}
