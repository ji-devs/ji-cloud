import { LitElement, html, css, customElement, property } from "lit-element";
import popupStyles from "./popup-styles";

const STR_CHANGE_PASSWORD = "Change Password";

@customElement("user-profile-reset-password-popup")
export class _ extends LitElement {
    static get styles() {
        return [
            popupStyles,
            css`
                .inputs {
                    display: grid;
                    grid-template-columns: repeat(2, 1fr);
                    grid-gap: 40px;
                    align-items: center;
                    margin: 48px 0;
                }
            `,
        ];
    }

    render() {
        return html`
            <slot name="close"></slot>
            <h2>${STR_CHANGE_PASSWORD}</h2>
            <div class="divider"></div>
            <div class="inputs">
                <slot name="inputs"></slot>
            </div>
            <div class="actions">
                <slot name="cancel"></slot>
                <slot name="save"></slot>
            </div>
        `;
    }
}
