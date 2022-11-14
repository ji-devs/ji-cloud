import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/entry/user/_common/footer/contact";
import "@elements/entry/user/email/buttons/email-send";
import "@elements/entry/user/_common/auth-page";

const STR_TITLE = "Heads up! We sent an email to ";
const STR_SUBTITLE1 = "Please check your inbox and click the verification button.";
const STR_SUBTITLE2 = "If you do not see an email from us, check your spam folder.";

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
                </div>
                <slot name="send"></slot>
                <slot name="submit"></slot>
                <footer-contact></footer-contact>
            </auth-page>
        `;
    }
}
