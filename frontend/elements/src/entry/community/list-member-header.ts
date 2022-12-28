import { LitElement, html, css, customElement, property } from "lit-element";
import { listItemStyles } from "./list-item-styles";

const STR_NAME = "Name";
const STR_LOCATION = "Location";
const STR_LANGUAGE = "Language";

@customElement("community-list-member-header")
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
            <span class="desktop-only">${STR_LOCATION}</span>
            <span class="desktop-only">${STR_LANGUAGE}</span>
        `;
    }
}
