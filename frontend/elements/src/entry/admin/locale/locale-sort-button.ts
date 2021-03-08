import { LitElement, html, css, customElement, property } from "lit-element";

export type FilterOption = [string, boolean];

@customElement("locale-sort-button")
export class _ extends LitElement {

    @property({type: Boolean, reflect: true})
    public sorted: boolean = false;

    static get styles() {
        return [css`
            button-text {
                display: grid;
                grid-template-columns: auto auto;
            }
            :host([sorted]) button-text::before {
                content: var(--sort-arrow);
                font-size: 15px;
                display: inline-block;
                margin-right: 3px;
            }
        `]
    }

    render() {
        return html`
            <button-text color="blue">
                Sort
            </button-text>
        `;
    }
}
