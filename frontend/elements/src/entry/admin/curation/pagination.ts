 import { LitElement, html, css, customElement } from "lit-element";

@customElement("table-pagination-jig")
export class _ extends LitElement {
    static styles = [
        css`
            :host {
                display: flex;
                flex-direction: row;
                align-items: center;
                grid-gap: 6px;
            }
        `,
    ];

    render() {
        return html`
            <span>Page</span>
            <slot name="back"></slot>
            <slot></slot>
            <slot name="next"></slot>
        `;
    }
}
