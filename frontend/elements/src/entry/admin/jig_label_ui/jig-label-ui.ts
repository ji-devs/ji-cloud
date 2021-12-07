import { LitElement, html, css, customElement, property } from "lit-element";
import jigFlex from "./jig-flex-css";

@customElement("jig-label-ui")
export class JigLabelUI extends LitElement {
  static styles = [
    jigFlex,
    css`
      .container {
        font-family: sans-serif;
        background: #f3f8fe;
        border: solid 1px #c4d9f7;
      }
      .headers {
        display: flex;
        justify-content: space-between;
        font-weight: 700;
        color: #2565d5;
      }
    `,
  ];

  @property({ attribute: false })
  headers: string[] = [
    "Jig Name",
    "Author",
    "Author's Badge",
    "Date",
    "Instruction Language",
    "Curators",
  ];

  render() {
    return html`
      <div class="container">
        <div class="headers">
          ${this.headers.map(
            (header) => html`<div class="flex">${header}</div>`
          )}
        </div>
        <slot></slot>
      </div>
    `;
  }
}
