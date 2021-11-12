import {
    LitElement,
    html,
    css,
    customElement,
    property,
    query,
} from "lit-element";

export type Bundle = [string, boolean];
export type Column = [string, boolean];

const STR_SELECT_COLUMNS = "Select columns to display";

@customElement("locale-page")
export class _ extends LitElement {
    public static get styles() {
        return [
            css`
                :host {
                    display: grid;
                    grid-template-columns: 200px auto 230px 140px;
                    row-gap: 40px;
                    column-gap: 50px;
                    padding: 50px;
                }

                .saving-indicator {
                    color: white;
                    background-color: #000000cc;
                    font-size: 12px;
                    font-weight: 700;
                    border-radius: 20px;
                    grid-column: 4;
                    position: fixed;
                    bottom: 10px;
                    right: 10px;
                }
                :host([saving]) .saving-indicator::after {
                    padding: 10px;
                    content: "Saving...";
                }

                .select-columns {
                    grid-column: 3;
                }
                .add-entry {
                    grid-column: 4;
                    justify-self: end;
                }

                .table {
                    grid-column: 1 / -1;
                    display: grid;
                }
                ::slotted([slot="rows"]) {
                    display: contents;
                }
                :host([sortOrder="asc"]) {
                    --sort-arrow: "⇩";
                }
                :host([sortOrder="desc"]) {
                    --sort-arrow: "⇧";
                }

                dialog {
                    border-radius: 8px;
                    border: solid #00000040 1px;
                }
                dialog::backdrop {
                    background-color: #00000060;
                }
                .column-selection-contents {
                    display: grid;
                    justify-items: end;
                }
            `,
        ];
    }

    @property({ type: Number })
    public columnsAmount: number = 11;

    @property({ type: Boolean, reflect: true })
    public saving = false;

    @property({ reflect: true })
    public sortOrder: "asc" | "desc" = "asc";

    @query("dialog", true)
    public dialog!: HTMLDialogElement;

    private showSelectColumns() {
        // this.dialog.showModal();
        // cant get @query to work, dono why
        (this.renderRoot.querySelector("dialog") as any)!.showModal();
    }

    private dismissSelectColumns() {
        // this.dialog.showModal();
        // cant get @query to work, dono why
        (this.renderRoot.querySelector("dialog") as any)!.close();
    }

    public render() {
        return html`
            <div class="saving-indicator"></div>
            <slot name="bundles"></slot>
            <button-rect
                color="blue"
                @click="${this.showSelectColumns}"
                class="select-columns"
            >
                ${STR_SELECT_COLUMNS}
                <!-- <img src="assets/select-columns-icon.png"> -->
            </button-rect>
            <slot class="add-entry" name="add-entry"></slot>
            <div
                class="table"
                style="grid-template-columns: repeat(${this
                    .columnsAmount}, auto)"
            >
                <slot name="rows"></slot>
            </div>
            <dialog>
                <div class="column-selection-contents">
                    <slot name="dialog-content"></slot>
                    <button-rect
                        @click="${this.dismissSelectColumns}"
                        color="blue"
                        >Okay</button-rect
                    >
                </div>
            </dialog>
        `;
    }
}
