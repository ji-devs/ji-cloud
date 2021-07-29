import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/popups/popup-body";

const STR_PREVIEW = "Preview";

@customElement("jig-preview-settings")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                .settings {
                    padding: 0 32px 44px 32px;
                }
                h3 {
                    margin: 0;
                    font-size: 18px;
                    font-weight: 800;
                    color: var(--dark-blue-4);
                    line-height: 1em;
                    margin-bottom: 20px;
                }
                .items {
                    display: grid;
                    row-gap: 24px;
                }
                ::slotted(label) {
                    display: flex;
                    column-gap: 18px;
                }
            `,
        ];
    }

    render() {
        return html`
            <div>
                <h3>${STR_PREVIEW}</h3>
                <div class="items">
                    <slot></slot>
                </div>
            </div>
        `;
    }
}
