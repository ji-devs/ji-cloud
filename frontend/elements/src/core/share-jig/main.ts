import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/popups/popup-body";
import "@elements/core/inputs/composed/switch-direction";
import "@elements/core/inputs/composed/switch";

const STR_SHARING_OPTIONS = "Sharing Options";

@customElement("share-jig-main")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                popup-body {
                    border-radius: 16px;
                    box-shadow: rgb(0 0 0 / 25%) 0px 3px 16px 0px;
                    background-color: #ffffff;
                }
                .body {
                    display: grid;
                    width: 305px;
                    max-width: 100vw;
                    box-sizing: border-box;
                }
                .settings {
                    background-color: var(--light-blue-2);
                    padding: 24px;
                    font-size: 14px;
                    display: grid;
                    gap: 12px;
                }
                .settings ::slotted(label) {
                    display: flex;
                    gap: 8px;
                }
                .options {
                    padding: 10px 24px 24px 24px;
                    display: grid;
                    gap: 6px;
                }
                .divider {
                    background-color: #d5e4ff;
                    height: 1px;
                }
            `,
        ];
    }

    render() {
        return html`
            <popup-body>
                <slot slot="close" name="close"></slot>
                <h3 slot="heading">${STR_SHARING_OPTIONS}</h3>
                <div class="body" slot="body">
                    <div class="settings">
                        <slot name="settings"></slot>
                    </div>
                    <div class="options">
                        <slot name="student"></slot>
                        <div class="divider"></div>
                        <slot name="other"></slot>
                    </div>
                </div>
            </popup-body>
        `;
    }
}
