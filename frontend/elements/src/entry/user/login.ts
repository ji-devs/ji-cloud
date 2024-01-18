import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/dividers/or-divider";
import "@elements/entry/user/_common/auth-page";

const STR_TITLE = "Login";
const STR_ACCOUNT = "Don't have an account yet?";

@customElement("user-login")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: grid;
                    height: 100dvh;
                }
                .inside-wrapper {
                    width: 296px;
                }
                ::slotted([slot=alert]) {
                    color: var(--dark-red-1);
                    background-color: var(--light-red-1);
                    border-radius: 3px;
                    margin: 0;
                    padding: 20px;
                    font-size: 16px;
                }
                h1 {
                    font-size: 30px;
                    font-weight: 900;
                    color: #5662a3;
                }
                ::slotted([slot="input"]) {
                    margin-top: 16px;
                }
                ::slotted([slot="password-forgot"]) {
                    text-align: end;
/* <<<<<<< HEAD:frontend/elements/src/entry/user/login/pages/landing.ts
                }
                ::slotted([slot="submit"]) {
                    margin-top: 40px;
                    margin-bottom: 24px;
                }
                .logo {
                    grid-column: 1;
                    position:absolute;
                    z-index:1;
                    top:0;
                    left:0;
                    padding: 25px;
                }
                .logo img-ui{
                    width: 85px;
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
=======
>>>>>>> 94a4024d4 (feat(fe/user/login): New sizes and element refactoring):frontend/elements/src/entry/user/login.ts */
                    margin-top: 16px;
                    margin-bottom: 56px;
                    display: block;
                }
                ::slotted([slot="submit"]) {
                    margin-top: 30px;
                    margin-bottom: 16px;
                }
                .spacer {
                    height: 16px;
                }
                footer {
                    margin-top: 16px;
                    font-size: 14px;
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

                <footer>
                    ${STR_ACCOUNT} <slot name="register"></slot>
                </footer>
            </auth-page>
        `;
    }
}
// <!-- <div class="img-ui">
// <div class="logo"><img src="core/page-header/logo.svg" /></div>
// </div>       -->