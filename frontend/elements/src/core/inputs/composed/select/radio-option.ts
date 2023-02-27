import { LitElement, html, css, customElement, property } from "lit-element";
import { nothing } from "lit-html";

@customElement("radio-button")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                base-option {
                    display: grid;
                    grid-template-columns: auto 1fr;
                    align-items: center;
                    column-gap: 10px;
                    padding: 4px 10px;
                }
                base-option:hover,
                base-option[active] {
                    background-color: var(--light-blue-2);
                }
                .checkbox {
                    display: inline-grid;
                    height: 16px;
                    width: 16px;
                    border-radius: 50%;
                    border: solid 1px #c7d3db;
                    box-sizing: border-box;
                }
                :host([selected]) .checkbox {
                    background-color: var(--main-blue);
                    color: white;
                    place-content: center;
                    font-size: 12px;
                    border: none;
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
                <span class="checkbox">
                    ${
                        this.selected
                            ? html`<fa-icon icon="fa-solid fa-circle-dot"></fa-icon>`
                        : nothing
                    }
                </span>
                <slot></slot>
                <!-- ${this.selected
                    ? html` <span class="check-mark">✔</span> `
                    : nothing} -->
        `;
    }
}
