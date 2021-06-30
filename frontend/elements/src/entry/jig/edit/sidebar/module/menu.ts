import { LitElement, html, css, customElement, property } from "lit-element";
import { classMap } from "lit-html/directives/class-map";
import { nothing } from "lit-html";
import "@elements/core/buttons/rectangle";

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

        .hidden {
            display: none;
        }


            `,
    ];
  }

  @property({type: Boolean})
  advanced:boolean = false;

  @property({type: Boolean})
  hideAdvancedSection:boolean = false;

  render() {
      const {advanced, hideAdvancedSection} = this;

      const advancedClasses = classMap({
          advanced: true,
          hidden: !advanced
      });
      const advancedButtonClasses = classMap({
          ["advanced-button"]: true,
          hidden: advanced
      });
      const advancedSectionClasses = classMap({
          hidden: hideAdvancedSection
      });

      return html`
          <section>
              <div class="lines">
                  <slot name="lines"></slot>
              </div>
              <div class="${advancedSectionClasses}">
                  <div class="separator"></div>

                  <div @click=${() => this.advanced = true} class="${advancedButtonClasses}">
                      <button-rect kind="text" color="blue">${STR_ADVANCED}</button-rect>
                  </div>
                  <div class="${advancedClasses}">
                      <slot name="advanced">
                      </slot>
                  </div>
            </div>

          </section>
      `;
  }
}
