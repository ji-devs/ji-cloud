import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/images/ui";

const STR_ADD_RESOURCE = "Add resource";

@customElement("jig-edit-publish-resource-button-add")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                button {
                    all: unset;
                    box-sizing: border-box;
                    display: grid;
                    grid-auto-flow: column;
                    justify-content: start;
                    column-gap: 8px;
                    border-radius: 12px;
                    border: solid 0px var(--dark-blue-3);
                    color: var(--dark-blue-3);
                    background-color: var(--light-blue-1);
                    padding: 16px;
                    cursor: pointer;
                    width: 100%;
                    font-size: 16px;
                    font-weight: 500;
                }
            `,
        ];
    }

    @property()
    label: string = "";

    render() {
        return html`
            <button>
                <fa-icon icon="fa-regular fa-plus"></fa-icon>
                ${STR_ADD_RESOURCE}
            </button>
        `;
    }
}
