import { MEDIA_UI } from "@utils/path";
import { LitElement, html, css, customElement, property } from "lit-element";
import {classMap} from "lit-html/directives/class-map";
import { noChange, nothing } from "lit-html";
import "@elements/core/buttons/expand";

@customElement("report-tree")
export class ReportTree extends LitElement {
  static get styles() {
    return [
      css`
      .indent-left-root {
          margin-left: 4px;
      }
      .indent-left-child {
        margin-left: 30px;
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

      .content-line {
        display: flex;
      }
    `,
    ];
  }

  @property({type: Boolean})
  hasChildren: boolean = false; 

  @property({type: Boolean})
  isChild : boolean = false; 


  render() {
    const { isChild} = this;

    const hasMarker = isChild; 

    const contentClasses = classMap({
      ["content-line"]: true,
      ["marker-offset-down"]: hasMarker,
    });

    const childrenClasses = classMap({
      ["indent-left-root"]: !isChild,
      ["indent-left-child"]: isChild,
    });

    return html`
        <div>
          <div class="content-line">
              ${hasMarker ? html`<div class="marker"></div>` : nothing}
              <div class="${contentClasses}"> 
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
