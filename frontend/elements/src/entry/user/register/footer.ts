import { LitElement, html, css, customElement, property } from "lit-element";

const STR_ACCOUNT = "Already have an account?";
const STR_LOGIN = "Log in";

@customElement("user-register-footer")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                footer {
                    font-size: 14px;
                }
            `,
        ];
    }

    render() {
        return html`
            <footer>
                ${STR_ACCOUNT} <button-rect kind="text" color="blue">${STR_LOGIN}</button-rect>
            </footer>
        `;
    }
}
