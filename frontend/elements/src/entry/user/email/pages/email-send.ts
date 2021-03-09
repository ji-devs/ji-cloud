import { MEDIA_UI } from "@utils/path";
import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/entry/user/_common/footer/contact";
import "@elements/entry/user/email/buttons/email-send";
import "@elements/entry/user/_common/base-page";

const STR_TITLE = "We Just Sent You an Email";
const STR_SUBTITLE = "Open the email and click on the Verification button";
const STR_SUBSUBTITLE = "It may have been filtered into the promotion or spam folders";

@customElement("page-email-send")
export class _ extends LitElement {
  static get styles() {
    return [
      css`
        .inside-wrapper {
          width: 624px;
        }
        h1 {
          font-size: 32px;
          font-weight: 900;
          color: #5662a3;
        }

        footer-contact {
          position: absolute;
          bottom: 20px;
        }

        ::slotted([slot="subtitle"]) {
          white-space: nowrap;
        }
      `,
    ];
  }

  @property()
  title: string = "";

  @property()
  subtitle: string = "";

  @property()
  hidden: boolean = true;

  render() {
    const { title, hidden, subtitle } = this;

    return html`
        <base-page>
          <h1>${STR_TITLE}</h1>
          <title-ji size="subMedium">${STR_SUBTITLE}<br/>${STR_SUBSUBTITLE}</title-ji>
          <slot name="change"></slot>
          <slot name="send"></slot>
          <slot name="submit"></slot>
          <footer-contact></footer-contact>
        </base-page>
    `;
  }
}
