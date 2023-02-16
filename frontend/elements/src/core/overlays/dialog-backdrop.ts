import { LitElement, html, css, customElement } from "lit-element";

@customElement("dialog-backdrop")
export class DialogOverlay extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    height: 100svh;
                    width: 100vw;
                    background: rgb(216 231 250 / .9);
                    display: grid;
                    place-content: center;
                    position: fixed;
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
