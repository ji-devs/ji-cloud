import { LitElement, html, css, customElement } from "lit-element";

@customElement("dialog-backdrop")
export class DialogOverlay extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    height: 100vh;
                    width: 100vw;
                    background: rgb(216 231 250 / .9);
                    display: grid;
                    place-content: center;
                }
            `,
        ];
    }

    render() {
        return html`
            <slot></slot>
        `;
    }
}
