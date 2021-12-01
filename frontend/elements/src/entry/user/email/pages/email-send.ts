import { LitElement, html, css, customElement } from "lit-element";
import "@elements/entry/user/_common/footer/contact";
import "@elements/entry/user/email/buttons/email-send";
import "@elements/entry/user/_common/base-page";

const STR_TITLE = "We just sent you an email";
const STR_SUBTITLE1 = "Please open the email and click on the Verification button. If you cannot see the email in your inbox, check your promotion or spam folders.";
const STR_SUBTITLE2 =
    "I didn’t receive an email. Please send again. (button)";

@customElement("page-email-send")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                .inside-wrapper {
                    width: 624px;
                }
                .title {
                    font-size: 32px;
                    font-weight: 900;
                    color: #5662a3;
                }
                .subtitle {
                    font-size: 20px;
                    font-weight: 300;
                    text-align: left;
                    color: #4a4a4a;
                    margin-bottom: 40px;
                }
                footer-contact {
                    position: absolute;
                    bottom: 20px;
                }
                ::slotted([slot="subtitle"]) {
                    white-space: nowrap;
                }
                ::slotted([slot="send"]) {
                    color: var(--dark-gray-5);
                    margin: 0;
                    font-size: 14px;
                }
            `,
        ];
    }

    render() {
        return html`
            <base-page>
                <div class="title">${STR_TITLE}</div>
                <div class="subtitle">
                    ${STR_SUBTITLE1}
                    <br />
                    ${STR_SUBTITLE2}
                </div>
                <slot name="send"></slot>
                <slot name="submit"></slot>
                <footer-contact></footer-contact>
            </base-page>
        `;
    }
}
