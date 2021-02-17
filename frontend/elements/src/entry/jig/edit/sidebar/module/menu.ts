import { LitElement, html, css, customElement, property } from "lit-element";
import { classMap } from "lit-html/directives/class-map";
import { nothing } from "lit-html";
import "@elements/core/buttons/text";

const STR_ADVANCED = "Advanced";

@customElement("jig-edit-sidebar-module-menu")
export class _ extends LitElement {
  static get styles() {
    return [
        css`
        section {
              width: 225px;
        }

        .lines, .advanced {
            display: flex;
            flex-direction: column;
        }
        .separator {
            display: block;
            margin-top: 18px;
            margin-bottom: 16px;
          width: 100%;
          height: 0;
          border: solid 1px #e4e6ed;
        }

        .advanced-button {
            display: flex;
            justify-content: center;
        }

            `,
    ];
  }


  render() {
      return html`
          <section>
              <div class="lines">
                  <slot name="lines"></slot>
              </div>
              <div class="separator"></div>
              <div class="advanced">
                  <slot name="advanced">
                    <div class="advanced-button">
                        <button-text color="blue">${STR_ADVANCED}</button-text>
                    </div>
                  </slot>
              </div>

          </section>
    `;
  }
}
