import { LitElement, html, css, customElement, property } from 'lit-element';
import {classMap} from "lit-html/directives/class-map";
import {nothing} from "lit-html";

const STR_ADD_WORDS = "Add Your Words";
const STR_INPUT_FOOTER = "2 to 14 words";

@customElement('step1-sidebar-duplicate')
export class _ extends LitElement {
  static get styles() {
      return [css`
          section {
              display: flex;
              flex-direction: column;
          }
          header {
              display: flex;
              justify-content: space-between;
          }

          .input-buttons {
              margin-top: 34px;
              margin-bottom: 24px;
              display: flex;
              justify-content: space-between;
          }
          .input-footer {
              margin-top: 24px;
              margin-bottom: 16px;
          font-size: 16px;
          text-align: center;
          color: var(--Light_Blue_5);
          }

          .btn-done {
            align-self: flex-end;
          }

    `];
  }

  render() {
      return html`
          <section>
              <header>
                  <div>${STR_ADD_WORDS}</div>
                  <div><slot name="clear"></slot></div>
              </header>
              <div class="input-buttons">
                  <slot name="input-buttons">
                  </slot>
              </div>
              <div class="input-widget">
                  <slot name="input-widget">
                  </slot>
                  <div class="input-footer">${STR_INPUT_FOOTER}</div>
              </div>
              <div class="btn-done">
                  <slot name="btn-done">
                  </slot>
              </div>
          </section>
      `
  }
}
