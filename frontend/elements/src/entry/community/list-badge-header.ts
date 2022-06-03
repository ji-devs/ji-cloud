import { LitElement, html, css, customElement, property } from "lit-element";

const STR_NAME = "Name";
const STR_MEMBERS = "Members";
const STR_ABOUT = "About";
const STR_STATUS = "Status";

@customElement("community-list-badge-header")
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
                    grid-template-columns: auto auto auto auto auto;
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
            <span>${STR_MEMBERS}</span>
            <span>${STR_ABOUT}</span>
            <span>${STR_STATUS}</span>
        `;
    }
}
