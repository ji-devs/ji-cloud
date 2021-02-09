import { MEDIA_UI } from "@utils/path";
import { LitElement, html, css, customElement, property } from "lit-element";
import { noChange, nothing } from "lit-html";
import "@elements/core/buttons/expand";
export type Mode = "checkbox" | "textInput" | "textDisplay";
@customElement("dropdown-tree")
export class DropdownTree extends LitElement {
  static get styles() {
    return [
      css`
    .main-wrapper{
        border-color:#e6f0ff;
        border-style:solid;
        border-width: 2px 2px 2px 8px;
        width:848px;
        min-height:48px;
        border-radius:12px;  
        
    }
    .bordergreen {
        border-left: solid 8px #6eca90;
    }
    .inside-wrapper{
        display:flex;
        align-items:center;
        padding-top:12px;
        
        
    }
    .text-wrapper{
        display:flex; 
        align-items:center;
    }
    ::slotted([slot=children]) {
        margin-top: 8px;
        margin-left:16px;
        
    }
    img {
        margin: 0 8px;
    }
    .open img{
        transform: rotate(90deg);
    }
    ul.closed {
        display: none;
    }
    ul{
        margin:0;
        padding:0;
    }
    p{
        line-height:0;
        margin:0;
    }
    .open .sidearrow{
        display:none;
    }
    .downarrow {
        display:none;
    }
    .open .downarrow {
        display:block;
    }
    ::slotted(*){
      display:flex;
      margin-left:40px;
    }
    .pointer {
      cursor: pointer;
    }
    `,
    ];
  }

  onExpandAllToggle(evt:CustomEvent) {
    const {value} = evt.detail;
    this.dispatchEvent(new Event(value ? "expand-all" : "collapse-all"));
  }

  @property()
  label: string = "";

  @property({type: Boolean})
  expanded: boolean = false; 

  @property()
  mode: Mode = "textDisplay";

  render() {
    const { label, expanded, mode} = this;

    return html`
      <div class="main-wrapper ${expanded ? "bordergreen open" : ""}">
        <div class="inside-wrapper">
          <div class="text-wrapper">
            <img-ui @click="${() => this.expanded = true}" class="sidearrow pointer" path="icon-chevron-categories-24-px.svg" alt=""></img-ui>
            <img-ui @click="${() => this.expanded = false}" class="downarrow pointer" path="icon-chevron-categories-24-px-active.svg" alt=""></img-ui>

            ${mode === "checkbox" ? renderCheckbox(this)
            : mode === "textInput" ? renderTextInput(this)
            : mode === "textDisplay" ? renderTextDisplay(this)
            : nothing
            }
            <button-expand @custom-toggle="${this.onExpandAllToggle.bind(this)}" .expanded="${expanded}" ></button-expand>

          </div>
        </div>
        <ul class="${expanded ? "open" : "closed"}">
          <slot></slot>
        </ul>
      </div>

        
    `;
  }
}
function renderCheckbox(self:DropdownTree) {
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

function renderTextDisplay(self:DropdownTree) {
  const {label} = self;
  return html`
    <div class="icon-wrapper">
      <div class="inside"></div>
    </div>
    <div>${label}</div>
    <slot name="menu-dropdown"></slot>
  `
}
function renderTextInput(self:DropdownTree) {
  const {label} = self;
    return html`
    <div class="icon-wrapper">
    <input class="textinput" type="text" value="${label}"/>
    <div class="inside"></div>
    </div>`
}