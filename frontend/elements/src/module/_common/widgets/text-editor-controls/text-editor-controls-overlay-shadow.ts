import { LitElement, html, css, customElement } from "lit-element";


@customElement("text-editor-controls-overlay-shadow")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: grid;
                    padding: 24px;
                    box-shadow: 0 3px 6px 0 rgba(0, 0, 0, 0.16);
                    border-radius: 14px;
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
