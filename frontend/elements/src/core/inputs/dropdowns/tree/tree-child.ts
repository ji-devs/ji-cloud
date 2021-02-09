import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/buttons/ellipses";
import "@elements/entry/admin/category/category-dropdown";

import { nothing } from "lit-html";
import { classMap } from "lit-html/directives/class-map";
export type Mode = "checkbox" | "textInput" | "textDisplay";

@customElement("dropdown-tree-child")
export class DropdownTreeChild extends LitElement {
  static get styles() {
    return [
      css`
      li{
          position:relative;
          display:flex;
          content-items:center;

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
      .ellipses-wrapper ::slotted([slot="menu-dropdown"]){
        display:none;
        position:absolute;
        left:50px;
      }
      .ellipses-wrapper:hover ::slotted([slot="menu-dropdown"]){
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
      .textinput{
        display: block;
          margin-top: -28px;
          margin-left: 25px;
      }
   
      .arrow {
        display: inline-block;
        cursor: pointer;
      }
      .arrow.expanded {
          transform: rotate(90deg);
      }
    `,
    ];
  }

  @property()
  label: string = "";

  @property({type: Boolean})
  expanded: boolean = false; 

  @property({type: Boolean})
  hasChildren: boolean = false; 

  @property()
  mode: Mode = "textDisplay";

  @property({ type: Boolean })
  hasMenu: boolean = true;

  render() {
    const { expanded, mode} = this;

    return html`
      <li class="titleoptions open">
        <div class="ellipses-wrapper">
            ${mode === "checkbox" ? renderCheckbox(this)
              : mode === "textInput" ? renderTextInput(this)
              : mode === "textDisplay" ? renderTextDisplay(this)
              : nothing
            }
        </div>
      
        <ul class="${expanded ? " open" : "closed" }">
          <slot></slot>
        </ul>
      </li>
    `;
  }
}

function renderCheckbox(self:DropdownTreeChild) {
  const {label} = self;

    return html`
    <div class="icon-wrapper">
      <input type="checkbox" />
      <div class="inside"></div>
    </div>
    <div>${label}</div>
    <slot name="menu-dropdown"></slot>
    `
}

function renderTextDisplay(self:DropdownTreeChild) {
  const {label} = self;
  return html`
    <div class="icon-wrapper">
      ${renderArrow(self)}
      <div class="inside"></div>
    </div>
    <div>${label}</div>
    <slot name="menu-dropdown"></slot>
  `
}
function renderTextInput(self:DropdownTreeChild) {
  const {label} = self;
    return html`
    <div class="icon-wrapper">
    ${renderArrow(self)}
    <input class="textinput" type="text" value="${label}"/>
    <div class="inside"></div>
    </div>`
}

function renderArrow(self:DropdownTreeChild) {
    const {expanded, hasChildren} = self;

    if(!hasChildren) {
      //THIS BREAKS WHEN MODE IS textInput
      return nothing;
    }
    const classes = classMap({
      arrow: true,
      expanded
    });

    return html`<img-ui 
      @click="${() => self.expanded = !self.expanded}" 
      path="icon-chevron-categories-24-px.svg" 
      alt=""
      class="${classes}"
    ></img-ui>`
}