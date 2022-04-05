import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/entry/user/_common/footer/contact";
import "@elements/core/titles/ji";
import "@elements/entry/user/_common/auth-page";

const STR_TITLE = "Change Email Account";
const STR_SUB = "This is the email that you filled in. You can change it now.";

@customElement("page-email-change")
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
                }
                .sub {
                    margin-bottom: 32px;
                    display: block;
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

    render() {
        return html`
            <auth-page img="entry/user/side/main.webp">
                <h1>${STR_TITLE}</h1>
                <title-ji color="black" class="sub">${STR_SUB}</title-ji>
                <div class="inside-wrapper">
                    <slot name="email"></slot>
                    <div class="spacer"></div>
                    <slot name="submit"></slot>
                </div>
                <footer-contact></footer-contact>
            </auth-page>
        `;
    }
}
