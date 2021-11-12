import { LitElement, html, css, customElement } from "lit-element";

@customElement("locale-row")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host(:nth-child(even)) {
                    --row-background-color: #e9ebf5;
                }
                :host(:nth-child(odd)) {
                    --row-background-color: #cfd5ea;
                }
            `,
        ];
    }

    render() {
        return html` <slot></slot> `;
    }
}
