import { MEDIA_UI } from "@utils/path";
import { LitElement, html, css, customElement, property } from "lit-element";
import { classMap } from "lit-html/directives/class-map";
import { noChange, nothing } from "lit-html";
import "@elements/core/buttons/expand";

export type ContainerMode = "multi-color" | "none";

@customElement("dropdown-tree")
export class DropdownTree extends LitElement {
  static get styles() {
    return [
      css`
      .container-multi-color {
          border-color:#e6f0ff;
          border-style:solid;
          border-width: 2px 2px 2px 8px;
          border-radius:12px;  
        padding-top: 13px;
        padding-bottom: 16px;
        padding-left: 2px;
      }

      .container-multi-color.expanded {
        background-color: white;
      }

      .container-multi-color.closed {
        background-color: #e6f0ff;
        border: solid 2px #e6f0ff;

        border-left: solid 8px #6eca90;
      }



      .indent-left-root {
        margin-left: 35px;
      }
      .indent-left-child {
        margin-left: 60px;
      }

      .marker-offset-down {
        margin-top: 10px;
      }

      .marker {
        border:solid 1px #c4dbff;
        border-right:none; 
        border-top:none;
        width:26px;
        height:24px;
      }

      .arrow {
        cursor: pointer;
        margin-right: 5px;
      }
      .arrowContainer {
        padding: 5px;
      }

      .children-visible {
        display: block;
      }
      .children-hidden {
        display: none;
      }

      .content-line {
        display: flex;
        align-items: center;
      }
    `,
    ];
  }

  onExpandAllToggle(evt:CustomEvent) {
    const {value} = evt.detail;
    this.dispatchEvent(new Event(value ? "expand-all" : "collapse-all"));
  }

  toggleExpand() {
    this.expanded = !this.expanded;
  }
  @property({type: Boolean})
  expanded: boolean = false; 

  @property({type: Boolean})
  hasChildren: boolean = false; 

  @property({type: Boolean})
  isChild : boolean = false; 

  @property()
  containerMode: ContainerMode = "multi-color";

  render() {
    const { expanded, containerMode, isChild} = this;

    const containerClasses = containerMode === "multi-color" ? multiColorClasses(this)
      : "";

    const hasMarker = isChild; 

    const contentClasses = classMap({
      ["content-line"]: true,
      ["marker-offset-down"]: hasMarker,
    });

    const childrenClasses = classMap({
      ["children-visible"]: expanded,
      ["children-hidden"]: !expanded,
      ["indent-left-root"]: !isChild,
      ["indent-left-child"]: isChild,
    });

    return html`
        <div class="${containerClasses}">
          <div class="content-line">
              ${hasMarker ? html`<div class="marker"></div>` : nothing}
              <div class="${contentClasses}"> 
                ${renderArrow(this)}
                <slot name="content"></slot>
              </div>
          </div>
          <div class="${childrenClasses}">
            <slot name="children"></slot>
          </div>
        </div>
    `
  }
}

function renderArrow(self: DropdownTree) {
  const {expanded, hasChildren} = self;


  if(!hasChildren) {
    return nothing;
  }

  return expanded ? html`<img-ui @click="${self.toggleExpand}" class="arrow" path="core/inputs/chevron-circle-down-green.svg" alt=""></img-ui>`
    : html`<div class="arrow arrowContainer" @click="${self.toggleExpand}"><img-ui path="core/_common/chevron-right-grey.svg" alt=""></img-ui></div>`
}

function multiColorClasses(self: DropdownTree) {
  const {expanded, isChild} = self;
 
  if(isChild) return "";

  return classMap({
    ["container-multi-color"]: true,
    expanded,
    closed: !expanded,
  });

}
