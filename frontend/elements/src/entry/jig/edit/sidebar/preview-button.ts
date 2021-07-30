import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/images/ui";

const STR_PREVIEW_JIG_LABEL = "Preview JIG";

@customElement("jig-edit-sidebar-preview-button")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                a {
                    cursor: pointer;
                    border: 0;
                    display: flex;
                    padding: 4px;
                    column-gap: 12px;
                    justify-content: space-between;
                    align-items: center;
                    font-size: 16px;
                    font-weight: 500;
                    color: var(--dark-gray-6);
                    font-family: inherit;
                    text-decoration: none;
                }
            `,
        ];
    }

    @property()
    href: string = "";

    render() {
        return html`
            <a href=${this.href} target="_blank">
                <img-ui path="entry/jig/preview.svg"></img-ui>
                ${STR_PREVIEW_JIG_LABEL}
            </a>
        `;
    }
}
