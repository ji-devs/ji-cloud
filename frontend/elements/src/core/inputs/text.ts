import { LitElement, html, css, customElement, property } from "lit-element";
import { classMap } from "lit-html/directives/class-map";
import { nothing } from "lit-html";

export type Mode = "passwordVisible" | "passwordHidden" | "text";

@customElement("input-text")
export class _ extends LitElement {
  static get styles() {
    return [
      css`
        .input-wrapper {
          position: relative;
          width: inherit;
          height: 64px;
          border: solid 1px #89b3ff;
          border-radius: 14px;
          padding: 8px 48px 8px 16px;
        }
        .errorwrapper {
          border: solid 1px #f00813;
          background-color: #fff4f4;
        }
        .errorwrapper input {
          background-color: #fff4f4;
        }

        input {
          outline: none;
          border: none;
          margin-top: 33px;
          width: inherit;
        }
        label {
          position: absolute;
          top: 0;
          left: 0;
          font-size: 16px;
          padding: 8px 0px 0px 16px;
          color: #5590fc;
        }
        .input-wrapper:active {
          border: solid 2px #5590fc;
          margin: -1px;
        }
        .input-wrapper:focus {
          border: solid 2px #5590fc;
          margin: -1px;
        }
        input {
          font-size: 16px;
          width: 100%;
        }

        .error {
          font-size: 14px;
          color: #f00813;
          margin-top: 4px;
          font-weight: 500;
          padding-left: 8px;
          display: block;
        }
        .instruction {
          font-size: 14px;
          color: #4a4a4a;
          margin-top: 4px;
          font-weight: 500;
          padding-left: 8px;
        }

        img-ui {
          position: absolute;
          top: 33%;
          right: 12px;
          cursor: pointer;
        }
      `,
    ];
  }

  @property()
  label: string = "";

  @property()
  value: string = "";

  // will also change the error wrapper internally
  @property()
  error: string = "";

  @property()
  help: string = "";

  @property()
  placeholder: string = "";

  // affects both icon display and input type
  @property()
  mode: Mode = "text";

  onPwToggle() {
    const {mode} = this;

    this.mode = mode === "passwordHidden" ? "passwordVisible"
      : mode === "passwordVisible" ? "passwordHidden"
      : mode;
  }

  render() {
    const { label, help, mode, placeholder, error, value } = this;

    const isError: boolean = error !== "";

    const isHelp: boolean = help !== "";

    const errorwrapper = isError ? "errorwrapper" : "";

    const inputType = mode === "passwordHidden" ? "password" : "text";

    return html`
      <div class="input-wrapper ${errorwrapper}">
        <input
          placeholder="${placeholder}"
          type="${inputType}"
          class=""
          value="${value}"
        />
        <label class="">${label}</label>
        ${mode !== "text" ? makeImage(mode, this.onPwToggle) : nothing}
      </div>

      ${isHelp ? html`<p class="instruction">${help}</p>` : nothing}
      ${isError ? html`<p class="error">${error}</p>` : nothing}
    `;
  }
}

function makeImage(mode: Mode, onPwToggle:(_:Event) => any) {
  const path =
    mode === "passwordVisible"
      ? "icn-show-idle.svg"
      : mode === "passwordHidden"
      ? "icn-hide-idle.svg"
      : "";

  return html`<img-ui path="${path}" @click="${onPwToggle}" ></img-ui>`;
}
