import { MEDIA_UI } from "@utils/path";
import { LitElement, html, css, customElement, property } from "lit-element";

const STR_TITLE = "Welcome to JI Family";
const STR_SUB = "You can now create, play, and share your content.";
const STR_SUBSUB = "We are here to help you in whatever you need.";

@customElement("page-register-complete")
export class _ extends LitElement {
  static get styles() {
    return [
      css`
        .content-wrapper {
          display: flex;
          justify-content: center;
          width: 100%;
          flex-direction: column;
          align-items: center;
        }
        .side-image {
          width: 100vw;
          height: 685px;
          background-color: #def4ff;
        }
        h1 {
          font-size: 32px;
          font-weight: 900;
          color: #5662a3;
        }
        .title {
          margin-bottom: 80px;
        }
        ::slotted([slot="button"]) {
          margin-top: 60px;
        }
      `,
    ];
  }

  render() {

    return html`
      <div class="wrapper">
        <div class="content-wrapper">
          <div class="title">
            <h1>${STR_TITLE}</h1>
            <title-ji size="subMedium">${STR_SUB}</title-ji>
            <title-ji size="subMedium">${STR_SUBSUB}</title-ji>
            <slot name="button"></slot>
          </div>
        </div>
        <div class="side-image"></div>
      </div>
    `;
  }
}
