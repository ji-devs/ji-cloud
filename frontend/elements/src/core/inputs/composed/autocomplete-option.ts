import { LitElement, html, css, customElement, property, query } from "lit-element";
import { nothing } from "lit-html";

// this element is currently only used for categories
// might be worth combining somehow with input-select-option
@customElement("input-autocomplete-option")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: grid;
                    grid-template-columns: auto 1fr;
                    align-items: center;
                    column-gap: 10px;
                    padding: 4px 10px;
                }
                :host(:hover) {
                    background-color: var(--light-blue-2);
                }
                .checkbox {
                    display: inline-grid;
                    height: 16px;
                    width: 16px;
                    border-radius: 3px;
                    border: solid 1px #c7d3db;
                    box-sizing: border-box;
                    cursor: pointer;
                }
                :host([selected]) .checkbox {
                    background-color: var(--main-blue);
                    color: white;
                    place-content: center;
                    font-size: 12px;
                    border: none;
                }
                .text {
                    font-weight: 300;
                }
                ::slotted(b) {
                    font-weight: 700;
                    color: var(--main-blue);
                }
            `,
        ];
    }

    @property({ type: Boolean, reflect: true })
    selected: boolean = false;

    private onCheck() {
        this.dispatchEvent(
            new CustomEvent("custom-selected", {
                detail: {
                    selected: this.selected,
                },
            })
        );
    }

    connectedCallback() {
        super.connectedCallback();

        this.setAttribute("tabindex", "0");
    }

    render() {
        return html`
            <span class="checkbox" @click=${this.onCheck}>
                ${
                    this.selected
                        ? html`<fa-icon icon="fa-solid fa-check"></fa-icon>`
                    : nothing
                }
            </span>
            <div class="text">
                <slot></slot>
            </div>
        `;
    }
}
