import { LitElement, html, css, customElement } from "lit-element";

@customElement("admin-curation-table-line")
export class _ extends LitElement {
    static styles = [
        css`
            :host {
                display: contents;
            }
            ::slotted(*) {
                padding: 5px;
                font-size: 14px;
                border: solid 1px #eaebef;
            }
        `,
    ];

    render() {
        return html`
            <slot></slot>
        `;
    }
}
