import { LitElement, html, css, customElement, property } from "lit-element";
import { nothing } from "lit-html";

const STR_PAGES = "Pages";

@customElement("community-pagination")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: grid;
                    grid-template-columns: repeat(3, auto);
                    justify-content: center;
                    align-items: center;
                    column-gap: 36px;
                    font-size: 16px;
                    font-weight: 500;
                    color: #383838;
                }
                ::slotted(fa-button) {
                    width: 36px;
                    height: 30px;
                    border-radius: 20px;
                    box-shadow: 0 0 6px 0 rgba(0, 0, 0, 0.08);
                    border: solid 1px var(--light-gray-2);
                    text-align: center;
                    color: #3558af;
                    display: inline-grid;
                    place-content: center;
                }
                ::slotted(fa-button[disabled]) {
                    filter: grayscale(1);
                    opacity: .5;
                }
                ::slotted(input) {
                    font: inherit;
                    color: inherit;
                    width: 30px;
                    text-align: right;
                    -moz-appearance: textfield;
                }
            `,
        ];
    }

    @property({ type: Number, reflect: true })
    total?: number;

    render() {
        return html`
            <slot name="back"></slot>
            ${
                typeof this.total === "number" ? html`
                    <div class="">
                        <slot name="active-page"></slot>
                        /
                        ${ this.total }
                        ${ STR_PAGES }
                    </div>
                ` : nothing
            }
            <slot name="forward"></slot>
        `;
    }
}
