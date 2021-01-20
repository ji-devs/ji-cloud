import { LitElement, html, css, customElement, property } from "lit-element";
import { classMap } from "lit-html/directives/class-map";

@customElement("vertical-full")
export class _ extends LitElement {
  static get styles() {
    return [
      css`
        div {
          width: 100%;
          height: 1px;
          background-color: #e5e7ef;
        }
      `,
    ];
  }

  render() {
    const {} = this;

    return html` <div></div> `;
  }
}
