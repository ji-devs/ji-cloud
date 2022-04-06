import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/dividers/or-divider";
import "@elements/entry/user/_common/auth-page";

const STR_TITLE = "Login";

@customElement("page-login-landing")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                .inside-wrapper {
                    width: 296px;
                }
                ::slotted([slot=alert]) {
                    color: var(--dark-red-1);
                    background-color: var(--light-red-1);
                    border-radius: 3px;
                    margin: 0;
                    padding: 26px;
                    font-size: 18px;
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
                ::slotted([slot="password-forgot"]) {
                    text-align: end;
                }
                ::slotted([slot="submit"]) {
                    margin-top: 40px;
                    margin-bottom: 24px;
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
                    margin-top: 16px;
                }
                ::slotted([slot="noaccount"]:last-child) {
                    margin-left: 4px;
                }
                ::slotted([slot="sub"]) {
                    white-space: nowrap;
                }
                ::slotted([slot="password-forgot"]) {
                    margin-top: 16px;
                    margin-bottom: 56px;
                    display: block;
                }
            `,
        ];
    }

    render() {
        return html`
            <auth-page img="entry/user/side/main.webp">
                <slot name="alert"></slot>
                <h1>${STR_TITLE}</h1>

                <div class="inside-wrapper">
                    <slot name="google"></slot>

                    <or-divider></or-divider>

                    <form @submit=${(evt: Event) => {evt.preventDefault()}}>
                        <slot name="email"></slot>
                        <div class="spacer"></div>
                        <slot name="password"> </slot>
                        <slot name="password-forgot"></slot>
                        <slot name="submit"></slot>
                    </form>
                </div>

                <slot name="footer"></slot>
            </auth-page>
        `;
    }
}
