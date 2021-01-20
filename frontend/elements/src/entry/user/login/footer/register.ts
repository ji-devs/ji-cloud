import { MEDIA_UI } from "@utils/path";
import { LitElement, html, css, customElement, property } from "lit-element";

const STR_ACCOUNT = "Don't have an account yet?";
const STR_LOGIN = "Register";

@customElement("footer-login-register")
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
            <title-ji color="black">${STR_ACCOUNT}</title-ji>
            &nbsp;
            <button-text color="blue">${STR_LOGIN}</button-text>
        </main>
    `;
  }
}
