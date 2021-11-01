import { LitElement, html, css, customElement, property } from "lit-element";

@customElement("fa-button")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                button {
                    all: unset;
                    cursor: pointer;
                }
            `,
        ];
    }

    @property()
    icon: string = "";

    render() {
        return html`
            <button>
                <fa-icon icon=${this.icon}></fa-icon>
            </button>
        `;
    }
}
