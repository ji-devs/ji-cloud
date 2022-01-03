import { LitElement, html, css, customElement, property } from "lit-element";

@customElement("admin-curation-table")
export class _ extends LitElement {
    static styles = [
        css`
            .table {
                display: grid;
                grid-template-columns: repeat(8, 1fr);
                font-family: sans-serif;
                background: #f3f8fe;
                border: solid 1px #c4d9f7;
            }
            .header-cell {
                border: solid 1px #eaebef;
                padding: 5px;
                color: #2565d5;
            }
            .pagination {
                display: grid;
                grid-template-columns: repeat(3, auto);
                column-gap: 6px;
                justify-items: center;
                margin: 0 auto;
                max-width: 200px;
            }
            ::slotted(button[slot=pagination]) {
                border: none;
                border-radius: 50%;
                height: 40px;
                width: 40px;
                display: inline-grid;
                place-content: center;
                font-size: 30px;
                cursor: pointer;
            }
            ::slotted(select[slot=pagination]) {
                font-size: 20px;
            }
        `,
    ];

    @property({ attribute: false })
    headers: string[] = [
        "Jig Name",
        "Author",
        "Author's Badge",
        "Date",
        "Instruction Language",
        "Curators",
        "Age Ranges",
        "Affiliation"
    ];

    render() {
        return html`
            <div class="table">
                <admin-curation-table-line>
                    ${this.headers.map(
                        (header) => html`<div class="header-cell">${header}</div>`
                    )}
                </admin-curation-table-line>
                <slot></slot>
            </div>
            <div class="pagination">
                <slot name="pagination"></slot>
            </div>
        `;
    }
}
