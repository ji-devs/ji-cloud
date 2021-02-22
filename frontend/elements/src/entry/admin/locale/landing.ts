import { LitElement, html, css, customElement, property, query } from "lit-element";

export type Bundle = [string, boolean];
export type Column = [string, boolean];

@customElement("locale-page")
export class _ extends LitElement {

    public static get styles() {
        return [css`
            :host {
                display: grid;
                grid-template-columns: 200px auto 180px 180px;
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
            .saving-indicator.visible::after {
                padding: 10px;
                content: 'Saving...';
            }

            .icon-button {
                height: 33px;
                display: flex;
                align-items: center;
                column-gap: 10px;
            }
            .icon-button button {
                background-color: transparent;
                border: 0;
                cursor: pointer;
                padding: 0;
                width: 50px;
            }
            .icon-button button img {
                max-height: 100%;
                max-width: 50px;
            }
            .icon-button.select-columns {
                grid-column: 3;
            }
            .icon-button.add-text {
                grid-column: 4;
            }

            .table {
                grid-column: 1 / -1;
                display: grid;
                grid-template-columns: repeat(11, auto);
            }
            ::slotted([slot=rows]) {
                display: contents;
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
                grid-template-columns: repeat(2, 200px);
            }
            .column-selection-contents hr {
                grid-column: 1 / -1;
                width: 100%;
            }
            .column-selection-contents header {
                grid-column: 1 / -1;
                text-align: center;
            }
            .column-selection-contents ul {
                list-style: none;
                padding: 0;
                margin: 0;
            }
            .column-selection-contents .actions {
                grid-column: 1 / -1;
                display: flex;
                justify-content: flex-end;
                column-gap: 10px;
            }
        `];
    }

    @property({type: Array})
    public bundles: Bundle[] = [];

    @property({type: Array})
    public columns: Column[] = []

    @property({type: Boolean})
    public saving = false;

    @property()
    public sortOrder: 'asc' | 'desc' = 'asc';

    @query('dialog', true)
    public dialog!: HTMLDialogElement;

    private onBundleSelect(e: Event) {
        const select = e.target as HTMLSelectElement;
        const options = select.options;
        const bundles = Array.from(this.bundles);
        this.bundles.forEach((b, i) => {
            b[1] = options[i].selected;
        });
        console.log(bundles);
        

        this.dispatchEvent(
            new CustomEvent("selected-bundle-change", {
                detail: bundles
            })
        );
    }

    private addEntry() {
        this.dispatchEvent(
            new CustomEvent("add-entry")
        );
    }

    private showSelectColumns() {
        // this.dialog.showModal();
        // cant get @query to work, dono why
        this.renderRoot.querySelector("dialog")!.showModal();
    }

    private dismissSelectColumns() {
        // this.dialog.showModal();
        // cant get @query to work, dono why
        this.renderRoot.querySelector("dialog")!.close();
    }

    public render() {
        return html`
            <div class="saving-indicator"></div>
            <select multiple @change="${this.onBundleSelect}">
                ${this.bundles.map(([bundleName, selected]) => {
                    return html`<option
                        .value="${bundleName}"
                        .selected="${selected}"
                    >
                        ${bundleName}
                    </option>`;
                })}
            </select>
            <div class="icon-button select-columns">
                <button @click="${this.showSelectColumns}"><img src="assets/select-columns-icon.png"></button>
                <span>{STR} Selectn columns to display</span>
            </div>
            <div class="icon-button add-text">
                <button @click="${this.addEntry}"><img src="assets/add-icon.png"></button>
                <span>{STR} Add a text</span>
            </div>
            <div class="table">
                <slot name="rows"></slot>
            </div>
            <dialog>
                <div class="column-selection-contents">
                    <header>Select Fields to Display</header>
                    <hr>
                    <ul class="columns-hidden">
                        ${this.columns
                            .filter(([_, visible]) => !visible)
                            .map(([columnName, _]) => html`<li>{{${columnName}}}</li>`)
                        }
                    </ul>
                    <ul class="columns-visible">
                        ${this.columns
                            .filter(([_, visible]) => visible)
                            .map(([columnName, _]) => html`<li>{{${columnName}}}</li>`)
                        }
                    </ul>
                    <hr>
                    <div class="actions">
                        <button @click="${this.dismissSelectColumns}">Cancel</button>
                        <button @click="${this.dismissSelectColumns}">Save</button>
                    </div>
                </div>
            </dialog>
        `;
    }
}
