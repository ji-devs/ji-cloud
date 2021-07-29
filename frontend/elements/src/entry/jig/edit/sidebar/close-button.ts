import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/images/ui";

@customElement("jig-edit-sidebar-close-button")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: inline-block;
                }
                button {
                    cursor: pointer;
                    border: 0;
                    padding: 0;
                    background-color: transparent;
                    display: grid;
                    height: 32px;
                    width: 32px;
                }
                img-ui {
                    grid-column: 1;
                    grid-row: 1;
                }
                .hover {
                    display: none;
                }
                button:hover .hover {
                    display: block;
                }
            `,
        ];
    }

    render() {
        return html`
            <button>
                <img-ui path="entry/jig/collapse.svg"></img-ui>
                <img-ui path="entry/jig/collapse-blue-bg.svg" class="hover"></img-ui>
            </button>
        `;
    }
}
