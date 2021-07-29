import { LitElement, html, css, customElement, property } from "lit-element";

@customElement("button-empty")
export class CircleButton extends LitElement {
    static get styles() {
        return [
            css`
                button {
                    all: inherit;
                    display: contents;
                    cursor: pointer;
                }
            `,
        ];
    }

    render() {
        return html`
            <button>
                <slot></slot>
            </button>
        `;
    }
}
