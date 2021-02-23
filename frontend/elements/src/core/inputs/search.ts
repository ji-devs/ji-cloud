import { LitElement, html, css, customElement, property } from "lit-element";
import { classMap } from "lit-html/directives/class-map";

@customElement("input-search")
export class _ extends LitElement {
  static get styles() {
    return [
      css`
        .wrapper {
          position: relative;
          width: 200px;
          height: 32px;
        }
        input {
          border: none;
          width: 200px;
          height: 32px;
          padding: 4px 40px 4px 16px;
          border-radius: 18px;
          border: solid 1px #e5e7ef;
          background-color: #f8f9fd;
          position: absolute;
          font-size: 16px;
        }
        input:focus {
          outline: none;
        }
        img-ui {
          position: absolute;
          right: 10px;
          z-index: 10;
          top: 5px;
        }
        input[type="search"]::-webkit-search-decoration,
        input[type="search"]::-webkit-search-cancel-button,
        input[type="search"]::-webkit-search-results-button,
        input[type="search"]::-webkit-search-results-decoration {
          display: none;
        }
      `,
    ];
  }

  @property()
  placeholder: string = "";

  @property()
  value: string = "";

  onKeyUp(evt:KeyboardEvent) {
    let { key } = evt;
    key = key.toLowerCase();
    if (key === "enter") {
        const {value} = (evt.target as any);
        this.value = value;

        this.dispatchEvent(new CustomEvent("custom-search", {
            detail: { value },
            composed: true,
            bubbles: true
        }))
    }
  }

  render() {
    const { placeholder, value } = this;
    return html`
      <div class="wrapper">
        <img-ui path="core/inputs/search.svg" alt="" class=""></img-ui>
        <input
            @keyup="${this.onKeyUp}"
          type="search"
          name=""
          value="${value}"
          placeholder="${placeholder}"
        />
      </div>
    `;
  }
}
