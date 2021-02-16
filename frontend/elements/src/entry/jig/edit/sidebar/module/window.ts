import { LitElement, html, css, customElement, property } from "lit-element";
import { classMap } from "lit-html/directives/class-map";
import { nothing } from "lit-html";
import "@elements/core/images/ui";
import "@elements/core/inputs/text-pencil";
import "@elements/core/buttons/icon";
import "@elements/core/buttons/text";

export type ModuleState = "empty" | "draft" | "complete";

const STR_EMPTY = "Drag\nactivity\nhere"

@customElement("jig-edit-sidebar-module-window")
export class _ extends LitElement {
  static get styles() {
    return [
        css`
            section {
                display: block;
                width: 218px;
                height: 123px;
                border-radius: 16px;
            }

            section.thumbnail {
                border: solid 2px #d8e7f9;
                background-color: var(--Light_Blue_2);
            }
            section.draft {
                border: solid 2px #d8e7f9;
                background-color: var(--Light_Blue_2);
            }
            section.empty {
                border: solid 0px #d8e7f9;
                background-color: var(--Light_Blue_5);
            }
            section.complete {
              border: solid 2px #c5e9d2;
              background-color: #d5f0de;
            }

            .contents {
                display: flex;
                align-items: center;
                justify-content: center;
                height: 100%;
            }
            .text {
                color: white;
                white-space: pre-wrap;
                font-size: 14px;
              font-weight: bold;
              font-stretch: normal;
              font-style: normal;
              line-height: 1.29;
              letter-spacing: normal;
              text-align: center;
              color: var(--white);
            }
            `,
    ];
  }

  @property()
  state:ModuleState = "draft";

  @property()
  thumbnail:string = "";

  render() {
      const {state, thumbnail} = this;

      let mainClasses = {main: true} as any;

      if(thumbnail !== "") {
          mainClasses.thumbnail = true;
      } else {
          mainClasses[state] = true;
      }

      return html`<section class="${classMap(mainClasses)}">
          ${ thumbnail !== "" ? renderThumbnail(thumbnail)
                : renderModule(state)
          }
          </section>
        `;
  }
}

function renderThumbnail(thumbnail:string) {
    return html`<div>TODO: ${thumbnail}</div>`
}

function renderModule(state:ModuleState) {
    if(state === "empty") {
        return html`<div class="contents"><div class="text">${STR_EMPTY}</div></div>`
    } else {
        const filename = state === "draft" ? "circle-pencil-blue.svg"
            : "circle-check-green.svg";
        
        const path = `core/buttons/icon/${filename}`;
        return html`<img-ui class="contents" path="${path}" />`
    }
}
