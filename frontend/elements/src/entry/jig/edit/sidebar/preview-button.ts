import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/images/ui";

const STR_PREVIEW_JIG_LABEL = "Preview JIG";

@customElement("jig-edit-sidebar-preview-button")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                button {
                    cursor: pointer;
                    border: 0;
                    background-color: transparent;
                    display: flex;
                    padding: 4px;
                    column-gap: 12px;
                    justify-content: space-between;
                    align-items: center;
                    font-size: 16px;
                    font-weight: 500;
                    color: var(--dark-gray-6);
                    font-family: inherit;
                }
            `,
        ];
    }

    render() {
        return html`
            <button>
                <img-ui path="entry/jig/preview.svg"></img-ui>
                ${STR_PREVIEW_JIG_LABEL}
            </button>
        `;
    }
}
