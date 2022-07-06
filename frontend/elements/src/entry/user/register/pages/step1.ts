import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/entry/user/_common/auth-page";

const STR_TITLE = "Sign Up - Step 1";

@customElement("page-register-step1")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                h1 {
                    font-size: 32px;
                    font-weight: 900;
                    color: #5662a3;
                }
                .inside-wrapper {
                    max-width: 650px;
                    display: grid;
                    grid-template-columns: 1fr 1fr;
                    align-items: start;
                    gap: 32px;
                }
                ::slotted([slot=username]),
                ::slotted([slot=checkbox]),
                ::slotted([slot=submit]) {
                    grid-column: 1 / -1;
                }
            `,
        ];
    }

    render() {
        return html`
            <auth-page img="entry/user/side/step-1.webp">
                <h1>${STR_TITLE}</h1>
                <div class="inside-wrapper">
                    <slot name="first-name"></slot>
                    <slot name="last-name"></slot>
                    <slot name="username"></slot>
                    <slot name="checkbox"></slot>
                    <slot name="submit"></slot>
                </div>
            </auth-page>
        `;
    }
}
