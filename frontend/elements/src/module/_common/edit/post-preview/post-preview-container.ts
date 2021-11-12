import { LitElement, html, css, customElement, property } from "lit-element";

@customElement("post-preview-container")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    position: absolute;
                    top: 0;
                    left: 0;
                    width: 100vw;
                    height: 100vh;
                    display: flex;
                    justify-content: center;
                    align-items: center;
                }
            `,
        ];
    }

    render() {
        return html` <slot></slot> `;
    }
}
