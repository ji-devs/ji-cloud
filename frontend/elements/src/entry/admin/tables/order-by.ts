import { LitElement, html, css, customElement } from "lit-element";

@customElement("table-order-by")
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
            <span>Sort by</span>
            <slot></slot>
        `;
    }
}
