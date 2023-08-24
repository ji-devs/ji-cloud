import { LitElement, html, css, customElement, property } from "lit-element";

@customElement("admin-table-user")
export class _ extends LitElement {
    static styles = [
        css`
            .controls {
                display: flex;
                align-items: center;
                grid-gap: 18px;
                padding: 10px;
            }
            .filter-controls {
                display: flex;
                grid-gap: 18px;
                justify-items: center;
                align-items: center;
            }
            ::slotted(button[slot=controls]) {
                border: none;
                border-radius: 50%;
                height: 40px;
                width: 40px;
                display: inline-grid;
                place-content: center;
                font-size: 30px;
                cursor: pointer;
            }
            ::slotted(select[slot=controls]) {
                font-size: 20px;
            }
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
        "Username",
        "First Name",
        "Family Name",
        "Email",
        "Badge",
        "Country",
        "State",
        "City",
        "School/Organization",
        "Signup Date",
        "Language",
        "Subscription",
        "Period End",
        "School Account",
        "Tier Override",
    ];

    render() {
        return html`
            <style>
                .table {
                    grid-template-columns: repeat(${this.headers.length}, 1fr);
                }
            </style>
            <div class="controls">
                <slot name="search"></slot>
                <div class="filter-controls">
                    <slot name="controls"></slot>
                </div>
            </div>
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
