import { MEDIA_UI } from "@utils/path";
import { LitElement, html, css, customElement, property } from "lit-element";
@customElement("or-divider")
export class _ extends LitElement {
  static get styles() {
    return [
      css`
        .wrapper {
          display: flex;
          align-items: center;
          width: 296px;
        }
        span {
          display: flex;
          align-items: center;
          font-size: 22px;
        }
        hr {
          width: 112px;
          border: solid 1px #dee1eb;
        }
      `,
    ];
  }

  @property()
  label: string = "";

  @property()
  path: string = "";

  render() {
    const { label, path } = this;

    return html`
      <div class="wrapper">
        <hr />
        <p>or</p>
        <hr />
      </div>
    `;
  }
}
