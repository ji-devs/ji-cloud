import { MEDIA_UI } from "@utils/path";
import { LitElement, html, css, customElement, property } from "lit-element";

@customElement("footer-contact")
export class _ extends LitElement {
  static get styles() {
    return [
      css`
        p {
          color: #5590fc;
        }
      `,
    ];
  }

  render() {
    const {} = this;

    return html`
      <p>
        If you need our help, kindly contact us on:
        <a href="mailto:info@jewishinteractive.org">info@jewishinteractive.org</a>
      </p>
    `;
  }
}
