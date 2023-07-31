import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/entry/user/_common/footer/contact";
import "@elements/entry/user/email/buttons/email-send";
import "@elements/entry/user/_common/auth-page";

const STR_TITLE = "Heads up! We sent an email to ";
const STR_SUBTITLE1 = "Your verification link will only be valid for 1 hour.";
const STR_SUBTITLE2 = "Please check your inbox and click the verification button.";
const STR_SUBTITLE3 = "If you do not see an email from us, check your spam folder.";

@customElement("page-email-send")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                .inside-wrapper {
                    max-width: 624px;
                }
                .title {
                    font-size: 30px;
                    font-weight: 900;
                    color: #5662a3;
                    /* needed for long email addresses on mobile */
                    word-wrap: break-word;
                    max-width: 90vw;
                }
                .subtitle {
                    font-size: 18px;
                    font-weight: 300;
                    text-align: left;
                    color: #4a4a4a;
                    margin-bottom: 30px;
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

    @property()
    email: string = "";

    render() {
        return html`
            <auth-page img="entry/user/side/confirm-email.webp">
                <div class="title">${STR_TITLE} ${this.email}</div>
                <div class="subtitle">
                    ${STR_SUBTITLE1}
                    <br />
                    ${STR_SUBTITLE2}
                    <br />
                    ${STR_SUBTITLE3}
                </div>
                <slot name="send"></slot>
                <slot name="submit"></slot>
                <footer-contact></footer-contact>
            </auth-page>
        `;
    }
}
