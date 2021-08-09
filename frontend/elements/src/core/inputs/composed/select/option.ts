import { LitElement, html, css, customElement, property } from "lit-element";
import { nothing } from "lit-html";
import "../../primitives/select/base-option";


@customElement("input-select-option")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                base-option {
                    display: flex;
                    justify-content: space-between;
                    column-gap: 10px;
                    padding: 4px 10px;
                }
                base-option:hover, base-option[active] {
                    background-color: var(--light-blue-2);
                }
                .check-mark {
                    color: var(--main-blue)
                }
            `,
        ];
    }

    @property({ type: Boolean, reflect: true })
    selected: boolean = false;

    onSelectedChange(e: CustomEvent) {
        this.selected = e.detail.selected;
        // re-dispatch event
        this.dispatchEvent(new CustomEvent("custom-selected", e));
    }

    render() {
        return html`
            <base-option ?selected=${this.selected} @custom-selected="${this.onSelectedChange}">
                <slot></slot>
                ${this.selected ? html`
                    <span class="check-mark">✔</span>
                ` : nothing}
            </base-option>
        `;
    }
}
