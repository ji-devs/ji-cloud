import { LitElement, html, css, customElement, property } from "lit-element";
import { nothing } from "lit-html";
import "@elements/core/popups/popup-body";

const STR_HEADER = "JIG Settings";

@customElement("jig-settings")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                .settings {
                    padding: 0 32px 44px 32px;
                    display: grid;
                    row-gap: 32px;
                }
            `,
        ];
    }

    render() {
        return html`
            <popup-body>
                <slot name="close" slot="close"></slot>
                <h2 slot="heading">${STR_HEADER}</h2>
                <div class="settings" slot="body">
                    <slot name="settings"></slot>
                </div>
            </popup-body>
        `;
    }
}
