import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/images/ui";

export type Kind = "settings" | "modules";

@customElement("jig-edit-sidebar-action-button")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                button {
                    cursor: pointer;
                    border: 0;
                    padding: 0;
                    background-color: transparent;
                    display: grid;
                    height: 32px;
                    width: 32px;
                    filter: opacity(.6);
                }
                button:hover {
                    filter: opacity(1);
                }
            `,
        ];
    }

    @property()
    kind: Kind = "settings";

    render() {
        return html`
            <button>
                <img-ui path="entry/jig/${this.kind}.svg"></img-ui>
            </button>
        `;
    }
}
