import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/entry/user/register/widgets/password-strength";
import { Strength as PasswordStrength } from "@elements/entry/user/register/widgets/password-strength";
import "@elements/core/dividers/or-divider";
import "@elements/entry/user/_common/auth-page";
const STR_TITLE = "Sign Up";

@customElement("page-register-start")
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
                    padding: 20px;
                    font-size: 16px;
                }
                h1 {
                    font-size: 30px;
                    font-weight: 900;
                    color: #5662a3;
                }
                ::slotted([slot="google"]) {
                    margin-bottom: 16px;
                }
                ::slotted([slot="input"]) {
                    margin-top: 16px;
                }
                ::slotted([slot="passwordreminder"]) {
                    text-align: end;
                }
                ::slotted([slot="submit"]) {
                    margin-top: 30px;
                    margin-bottom: 16px;
                }
/* <<<<<<< HEAD
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
=======
>>>>>>> 3fa277a37 (feat(fe/user/register): New sizes) */
                .spacer {
                    height: 16px;
                }
                .password-wrapper {
                    position: relative;
                }
                .password-wrapper div {
                    position: absolute;
                    top: 33%;
                    right: -76px;
                }
            `,
        ];
    }

    @property()
    passwordStrength: PasswordStrength = "none";

    render() {
        const { passwordStrength } = this;

        return html`
            <auth-page img="entry/user/side/main.webp">
                <div class="logo">
                    <img-ui path="core/page-header/logo.svg"></img-ui>
                </div>
                <slot name="alert"></slot>
                <h1>${STR_TITLE}</h1>
                <div class="inside-wrapper">
                    <slot name="google"></slot>
                    <or-divider slot="divider"></or-divider>
                    <form
                        @submit=${(evt: Event) => {
                            evt.preventDefault();
                        }}
                    >
                        <slot name="email"></slot>
                        <div class="spacer"></div>
                        <div class="password-wrapper">
                            <slot name="password"></slot>
                            <password-strength
                                strength="${passwordStrength}"
                            ></password-strength>
                            <div>${strengthText(passwordStrength)}</div>
                        </div>
                        <p>&nbsp;</p>
                        <slot name="submit"></slot>
                    </form>
                    <p></p>
                </div>
                <slot name="footer"></slot>
            </auth-page>
        `;
    }
}

function strengthText(mode: PasswordStrength) {
    const strengthlabel =
        mode === "weak"
            ? "Weak"
            : mode === "average"
            ? "Average"
            : mode === "strong"
            ? "Strong"
            : "";

    return html`<p>${strengthlabel}</p>`;
}
