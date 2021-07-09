import { MEDIA_UI } from "@utils/path";
import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/entry/user/_common/base-page";
import {Strength as PasswordStrength} from "@elements/entry/user/register/widgets/password-strength";

const STR_TITLE = "Create a New password";
//const STR_AFTER = "You’ll be logged in automatically after this";
const STR_AFTER = "You will need to log in after this";

@customElement("page-password-reset")
export class _ extends LitElement {
  static get styles() {
    return [
      css`
        .inside-wrapper {
          width: 296px;
        }
        h1 {
          font-size: 32px;
          font-weight: 900;
          color: #5662a3;
        }
        ::slotted([slot="google"]) {
          margin-bottom: 20px;
        }
        ::slotted([slot="input"]) {
          margin-top: 20px;
        }
        ::slotted([slot="passwordreminder"]) {
          text-align: end;
        }
        ::slotted([slot="submit"]) {
          margin-top: 40px;
          margin-bottom: 24px;
          display:block;
        }

        .spacer {
          height: 20px;
        }
        .text-hidden {
          display: none;
        }
        .password-wrapper {
          position: relative;
        }
        .password-wrapper div {
          position: absolute;
          top: 33%;
          right: -76px;
        }
        ::slotted([slot="contact"]) {
          position: absolute;
          bottom: 20px;
          white-space: nowrap;
        }
        .account-wrapper {
          display: flex;
          align-items: center;
          margin-top:24px;
        }
        ::slotted([slot="noaccount"]:last-child) {
          margin-left: 4px;
        }
        ::slotted([slot="sub"]) {
          white-space: nowrap;
        }
      `,
    ];
  }

  @property()
  passwordStrength: PasswordStrength = "none";

  render() {

    const {passwordStrength} = this;

    return html`
        <base-page>
          <h1>${STR_TITLE}</h1>
          <div class="inside-wrapper">
            <div class="password-wrapper">
            <password-strength strength ="${passwordStrength}"></password-strength>

              <slot name="password"> </slot>
              <div>${strengthText(passwordStrength)}</div>
            </div>
            <slot name="submit"></slot>
          </div>
          <div class="account-wrapper">
            <title-ji color="black">${STR_AFTER}</title-ji>
          </div>
        </base-page>
    `;
  }
}


function strengthText(mode: PasswordStrength) {
    const strengthlabel = mode === "weak" ? "Weak"
      : mode === "average" ? "Average"
      : mode === "strong" ? "Strong"
      : "";
    
      return html`<p>${strengthlabel}</p>`
    
  }
