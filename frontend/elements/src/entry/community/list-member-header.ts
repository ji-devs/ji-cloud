import { LitElement, html, css, customElement, property } from "lit-element";

const STR_NAME = "Name";
const STR_LOCATION = "Location";
const STR_LANGUAGE = "Language";

@customElement("community-list-member-header")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    border-radius: 8px;
                    min-height: 48px;
                    font-size: 16px;
                    color: var(--dark-gray-5);
                    background-color: #ffecad;
                    border-radius: 8px;
                    display: grid;
                    align-items: center;
                    justify-content: space-between;
                }
                .name {
                    grid-column: 2;
                }
            `,
        ];
    }

    render() {
        return html`
            <span class="name">${STR_NAME}</span>
            <span>${STR_LOCATION}</span>
            <span>${STR_LANGUAGE}</span>
        `;
    }
}
