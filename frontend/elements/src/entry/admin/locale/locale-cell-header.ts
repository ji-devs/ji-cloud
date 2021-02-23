import { LitElement, html, css, customElement, property } from "lit-element";
import { nothing } from "lit-html";

export type FilterOption = [string, boolean];

@customElement("locale-cell-header")
export class _ extends LitElement {

    @property()
    public label: string = "";
    
    @property({type: Boolean})
    public sortable: boolean = false;
    
    @property({type: Boolean, reflect: true})
    public sorted: boolean = false;

    @property({type: Array, reflect: true})
    public filterOptions: FilterOption[] | null = null;

    @property({type: Boolean, reflect: true})
    public adminOnly: boolean = false;


    static get styles() {
        return [css`
            :host {
                border: solid white 2px;
                display: flex;
                flex-direction: column;
                justify-content: flex-end;
                row-gap: 4px;
            }

            .filter-select {
                align-self: flex-end;
            }

            .sort-button {
                border: 0;
                background-color: transparent;
                padding: 3px;
                margin: 0;
                cursor: pointer;
                align-self: flex-end;
            }
            .sort-button:hover {
                text-decoration: underline;
            }
            :host([sorted]) .sort-button::before {
                content: var(--sort-arrow);
                font-size: 15px;
                display: inline-block;
                margin-right: 3px;
            }

            .main-section {
                background-color: #4472c4;
                color: white;
                text-align: left;
                padding: 10px 10px;
                font-size: .8em;
                min-height: 18px;
            }
            :host([adminonly]) .main-section {
                background-color: #afabab;
            }
        `]
    }


    private onFilter(e: Event) {
        const select = e.target as HTMLSelectElement;
        const options = select.options;
        this.filterOptions!.forEach((o, i) => {
            o[1] = options[i].selected;
        });

        this.dispatchEvent(
            new CustomEvent("filter", {
                detail: this.filterOptions
            })
        );
    }

    private addSort() {
        this.dispatchEvent(
            new CustomEvent("sort")
        );
    }

    render() {
        return html`
            ${ this.filterOptions && (
                
                html`<select multiple class="filter-select" @change="${this.onFilter}">${this.filterOptions.map(([option, selected]) => {
                    return html`<option
                        .value="${option}"
                        .selected="${selected}"
                    >
                        ${option}
                    </option>`;
                })}</select>`
            ) }
            ${ this.sortable ? 
                html`<button @click="${this.addSort}" class="sort-button">Sort</button>`
            : 
                nothing
            }
            <span class="main-section">${this.label}</span>
        `;
    }
}
