import { LitElement, html, css, customElement, property } from "lit-element";

@customElement("admin-school-user-table")
export class _ extends LitElement {
    static styles = [
        css`
            .table {
                display: grid;
                font-family: sans-serif;
                border: solid 1px #c4d9f7;
            }
            .header-cell {
                background-color: #f3f8fe;
                border: solid 1px #eaebef;
                padding: 5px;
                color: #2565d5;
            }
        `,
    ];

    @property({ attribute: false })
    headers: string[] = [
        "Email",
        "Verified",
        "Admin",
        "Actions"
    ];

    render() {
        return html`
            <style>
                .table {
                    grid-template-columns: repeat(${this.headers.length}, 1fr);
                }
            </style>
            <div class="table">
                <admin-table-line>
                    ${this.headers.map(
            (header) => html`<div class="header-cell">${header}</div>`
        )}
                </admin-table-line>
                <slot></slot>
            </div>
        `;
    }
}
