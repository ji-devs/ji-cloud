import { MEDIA_UI } from "@utils/path";
import { LitElement, html, css, customElement, property } from "lit-element";

const STR_ACCOUNT = "Already have an account?";
const STR_LOGIN = "Login";

@customElement("footer-register-login")
export class _ extends LitElement {
  static get styles() {
    return [
      css`
        main {
          display: flex;
          align-items: center;
          margin-top: 16px;
        }
      `,
    ];
  }

  render() {
    return html`
        <main>
            <title-ji color="black" weight="normal">${STR_ACCOUNT}</title-ji>
            &nbsp;
            <button-rect kind="text" color="blue">${STR_LOGIN}</button-rect>
        </main>
    `;
  }
}
