import { LitElement, html, css, customElement, property } from "lit-element";
import { classMap } from "lit-html/directives/class-map";
import { nothing } from "lit-html";
import "@elements/core/images/ui";
import {ModuleKind, STR_MODULE_DISPLAY_NAME} from "@elements/module/_common/types";

@customElement("jig-edit-module-card")
export class _ extends LitElement {
  static get styles() {
    return [
        css`
            section {
              width: 248px;
              height: 224px;
              padding: 24px 0 0;
              border-radius: 16px;
              box-shadow: 0 3px 6px 0 rgba(0, 0, 0, 0.16);
              background-color: var(--white);
              display: grid;
              grid-template-rows: 1fr 40px;
            }
            .bottom {
                display: flex;
                justify-content: center;
                align-items: center;
                text-align: center;
                font-size: 16px;
              border-bottom-left-radius: 16px;
              border-bottom-right-radius: 16px;
              font-weight: 500;
              font-stretch: normal;
              font-style: normal;
              line-height: 1.25;
              letter-spacing: normal;
              text-align: center;
              color: var(--dark-gray-6);
              background-color: var(--light-blue-2);
            }

            .top {
                display: flex;
                justify-content: center;
                align-items: center;

            }
        `,
    ];
  }

  onEnter() {
    this.hover = true;
  }

  onLeave() {
    this.hover = false;
  }

  @property()
  module:ModuleKind = "memory";

  @property({type: Boolean})
  drag:boolean = false;

  @property({type: Boolean})
  hover:boolean = false;

  render() {
      const {module, drag, hover} = this;

      const iconSuffix = drag ? `-drag`
        : hover ? `-hover`
            : ``;

    const iconPath = `entry/jig/modules/large/${module}${iconSuffix}.svg`
    return html`

        <section @mouseenter="${this.onEnter}" @mouseleave="${this.onLeave}">
              <div class="top">
                  <img-ui path="${iconPath}"></img-ui>
                </div>
                <div class="bottom">
                    ${STR_MODULE_DISPLAY_NAME[module]}
              </div>
          </section>
        `;
  }
}
