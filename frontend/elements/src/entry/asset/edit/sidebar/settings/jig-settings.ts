import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/popups/popup-body";

const STR_HEADER = "JIG Settings";
const STR_CREATOR = "Creator";

@customElement("jig-settings")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                popup-body {
                    width: 424px;
                    display: block;
                }
                .settings {
                    padding: 0 32px 44px 32px;
                }
                h3 {
                    margin: 0;
                    font-size: 18px;
                    font-weight: 800;
                    color: var(--dark-blue-4);
                }
                .creator-items {
                    margin: 16px 0;
                    display: grid;
                    row-gap: 16px;
                }
                .divider {
                    background-color: #d5e4ff;
                    margin: 24px 0;
                    height: 1px;
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
                    <div class="creator">
                        <h3>${STR_CREATOR}</h3>
                        <div class="creator-items">
                            <slot name="creator"></slot>
                        </div>
                    </div>
                    <div class="divider"></div>
                    <slot name="preview"></slot>
                </div>
            </popup-body>
        `;
    }
}
