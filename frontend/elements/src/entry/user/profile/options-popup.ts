import { LitElement, html, css, customElement, property } from "lit-element";
import popupStyles from "./popup-styles";

@customElement("user-profile-options-popup")
export class _ extends LitElement {
    static get styles() {
        return [
            popupStyles,
            css`
                .options {
                    display: grid;
                    row-gap: 5px;
                }
                .options h4 {
                    margin: 0;
                    font-weight: 500;
                    margin-bottom: 10px;
                    color: var(--main-blue);
                }
                .actions {
                    justify-content: flex-end;
                }
            `,
        ];
    }

    @property()
    header: string = "";

    @property()
    subheader: string = "";

    render() {
        return html`
            <slot name="close"></slot>
            <h2>${this.header}</h2>
            <div class="divider"></div>
            <div class="options">
                <h4>${this.subheader}</h4>
                <slot name="options"></slot>
            </div>
            <div class="divider"></div>
            <div class="actions">
                <slot name="done"></slot>
            </div>
        `;
    }
}
