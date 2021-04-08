import { LitElement, html, css, customElement, property } from "lit-element";
import { classMap } from "lit-html/directives/class-map";

@customElement("input-form-textarea")
export class _ extends LitElement {
  static get styles() {
    return [
      css`
        .wrapper {
          margin-bottom: 16px;
          border-radius: 14px;
          border: solid 1px #89b3ff;
          margin-top: 40px;
          padding: 8px 16px;
          background-color: white;
        }
        label {
          padding-left: 8px;
        }
        span {
          color: #5590fc;
        }
        .textarea-wrapper {
          display: flex;
          align-items: center;

          position: relative;
        }

        input {
          outline: none;
          border: none;
          font-size: 16px;
          padding: 0 8px;
          width: 100%;
        }
        focus {
          outline: none;
          border: solid 2px #5590fc;
        }
        wrapper:active {
          border: solid 2px #5590fc;
        }
        ::placeholder {
          color: #a1a8ad;
        }
        img {
          position: absolute;
          right: -10px;
        }
        textarea {
          resize: none;
          width: 100%;
          outline: none;
          background: transparent;
          appearance: none;
          padding-left: 8px;
          font-family: Poppins;
          font-size: 16px;
          border: none;
        }
      `,
    ];
  }

  @property()
  label: string = "";

  @property()
  placeholder: string = "";

  @property()
  value: string = "";

  @property({ type: Number })
  rows: number = 10;

  onInput() {
      const input = this.shadowRoot?.getElementById("input") as HTMLInputElement;
        this.dispatchEvent(
            new CustomEvent("custom-input", {
                detail: { value: input.value},
            })
        );
  }
  onBlur() {
      const input = this.shadowRoot?.getElementById("input") as HTMLInputElement;
        this.dispatchEvent(
            new CustomEvent("custom-change", {
                detail: { value: input.value},
            })
        );
  }
  render() {
    const { label, value, rows, placeholder } = this;

    return html`
      <div class="wrapper">
        <label for="name" class="">
          <span class="text-jibuttonBlue">${label}</span>
          <div class="textarea-wrapper">
            <textarea rows="${rows}" id="input" type="text" placeholder="${placeholder}" @blur="${this.onBlur}" @input="${this.onInput}" .value="${value}"></textarea>
          </div>
        </label>
      </div>
    `;
  }
}
