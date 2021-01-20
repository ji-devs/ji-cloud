import { MEDIA_UI } from "@utils/path";
import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/titles/ji";

const STR_TITLE = "Sign Up - Step 3";
const STR_SUBTITLE = "We want to tailor the content that you find to your interests and needs.";
const STR_SUBSUBTITLE = "You can select as many as you like now and edit it later it in your profile page";

@customElement("page-register-step3")
export class _ extends LitElement {
  static get styles() {
    return [
      css`
        .wrapper {
          display: flex;
        }
        .inside-wrapper {
          width: 624px;
        }
        .side-image {
          width: 480px;
          min-width: 300;
          min-height: 100vh;
          background-color: #def4ff;
          background-image: url("https://i.ibb.co/g9N7MLy/shapes-1.png");
          background-repeat: no-repeat;
          background-attachment: inherit;
          background-position: center;
        }
        .content-wrapper {
          padding: 80px;
          width: 867px;
          position: relative;
        }
        h1 {
          font-size: 32px;
          font-weight: 900;
          color: #5662a3;
        }
        ::slotted([slot="contact"]) {
          position: absolute;
          bottom: 20px;
        }

        .subtitle {
          white-space: nowrap;
        }
      `,
    ];
  }

  render() {

    return html`
      <div class="wrapper">
        <div class="side-image"></div>
        <div class="content-wrapper">
          <h1>${STR_TITLE}</h1>

          <title-ji class="subtitle" size="subMedium">${STR_SUBTITLE}</title-ji>
          <title-ji class="subtitle" size="subMedium">${STR_SUBSUBTITLE}</title-ji>
          <slot name="main"></slot>
          <slot name="submit"></slot>
        </div>
      </div>
    `;
  }
}