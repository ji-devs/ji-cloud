import { LitElement, html, css, customElement } from "lit-element";

@customElement("admin-curation-table-line")
export class _ extends LitElement {
    static styles = [
        css`
            :host {
                display: contents;
            }
            /* :host(:nth-child(even)) ::slotted(*) {
                background-color: #f2f2f2;
            } */
            :host(:hover) ::slotted(*) {
                background-color: rgb(233 233 233);
            }
            ::slotted(*) {
                padding: 5px;
                font-size: 14px;
                border: solid 1px #eaebef;
            }
            ::slotted(a) {
                cursor: pointer;
            }
            /* keep the :host part to increase specificity because of :host(:hover) */
            :host ::slotted(a:hover) {
                text-decoration: underline;
                color: var(--dark-blue-5);
            }
        `,
    ];

    render() {
        return html`
            <slot></slot>
        `;
    }
}
