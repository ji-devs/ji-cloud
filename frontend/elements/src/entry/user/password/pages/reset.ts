import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/entry/user/_common/auth-page";
import { Strength as PasswordStrength } from "@elements/entry/user/register/widgets/password-strength";

const STR_TITLE = "Create a new password";
//const STR_AFTER = "You’ll be logged in automatically after this";
const STR_AFTER = "You will need to log in again after changing your password";

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
                ::slotted([slot="submit"]) {
                    margin-top: 40px;
                    margin-bottom: 24px;
                    display: block;
                }
                .password-wrapper {
                    position: relative;
                }
                .password-wrapper div {
                    position: absolute;
                    top: 33%;
                    right: -76px;
                }
                .account-wrapper {
                    display: flex;
                    align-items: center;
                    margin-top: 24px;
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
                <h1>${STR_TITLE}</h1>
                <div class="inside-wrapper">
                    <form
                        @submit=${(evt: Event) => {
                            evt.preventDefault();
                        }}
                    >
                        <div class="password-wrapper">
                            <password-strength
                                strength="${passwordStrength}"
                            ></password-strength>

                            <slot name="password"> </slot>
                            <div>${strengthText(passwordStrength)}</div>
                        </div>
                        <slot name="submit"></slot>
                    </form>
                </div>
                <div class="account-wrapper">
                    <title-ji color="black">${STR_AFTER}</title-ji>
                </div>
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
