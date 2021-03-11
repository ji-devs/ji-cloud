import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/images/ui";

const STR_LABEL ="Hebrew keyboard";
export type MODE = "keyboard" | "dicta" | "sefaria";

@customElement("button-sidebar")
export class _ extends LitElement {
  static get styles() {
    return [
      css`
        :host {
            cursor: pointer;
            display: inline-block;
              border-radius: 8px;
              border: solid 1px var(--Light_Blue_5);
              background-color: var(--white);
        }

        section {
            display: flex;
            align-items: center;
            justify-content: center;
              height: 64px;
        }
        .keyboard {
            display: flex;
            align-items: center;
            justify-content: center;
            gap: 15px;
            margin: 0 20px;
        }

        .keyboard > img-ui {
            margin-top: 5px;
        }

        .dicta {
            margin: 0 23px;
        }

        .sefaria {
            margin: 0 17px;
        }
        .label {
          font-family: Poppins;
          font-size: 16px;
          letter-spacing: normal;
          text-align: left;
          color: #4a4a4a;
        }
      `,
    ];
  }

  @property()
  mode:MODE = "keyboard";

  render() {
      const {mode} = this;

      return html`<section>
          ${mode === "keyboard" ? renderKeyboard()
              : mode === "dicta" ? renderDicta()
              : renderSefaria()
          }
      </section>
  `;
  }
}

function renderKeyboard() {
    return html`
        <div class="keyboard">
            <div class="label">${STR_LABEL}</div>
            <img-ui path="core/buttons/keyboard.svg"></img-ui>
        </div>
    `;
}
function renderDicta() {
    return html`<img-ui class="dicta" path="core/buttons/dicta.svg"></img-ui>`;
}
function renderSefaria() {
    return html`<img-ui class="sefaria" path="core/buttons/sefaria.png"></img-ui>`;
}
