import { LitElement, html, css, customElement, property } from "lit-element";
import { listItemStyles } from "./list-item-styles";

const STR_NAME = "Name";
const STR_MEMBERS = "Members";
const STR_ABOUT = "About";
const STR_STATUS = "Status";

@customElement("community-list-circle-header")
export class _ extends LitElement {
    static get styles() {
        return [
            listItemStyles,
            css`
                :host {
                    display: none;
                }
                @media (min-width: 1024px) {
                    :host {
                        border-radius: 6px;
                        min-height: 36px;
                        font-size: 14px;
                        color: var(--dark-gray-5);
                        background-color: #ffecad;
                    }
                    .name {
                        grid-column: 2;
                    }
                }
            `,
        ];
    }

    render() {
        return html`
            <span class="name">${STR_NAME}</span>
            <span class="desktop-only">${STR_MEMBERS}</span>
            <span class="desktop-only">${STR_ABOUT}</span>
            <span class="desktop-only">${STR_STATUS}</span>
        `;
    }
}
