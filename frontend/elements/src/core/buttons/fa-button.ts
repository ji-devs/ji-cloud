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
                :host([disabled]) {
                    pointer-events: none;
                }
            `,
        ];
    }

    @property()
    icon: string = "";

    @property({ type: Boolean, reflect: true })
    disabled: boolean = false;

    render() {
        return html`
            <button ?disabled="${this.disabled}">
                <fa-icon icon=${this.icon}></fa-icon>
            </button>
        `;
    }
}
