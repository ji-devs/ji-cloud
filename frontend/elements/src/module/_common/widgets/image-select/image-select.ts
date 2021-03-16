import { LitElement, html, css, customElement, property} from 'lit-element';
import { nothing } from "lit-html";

@customElement('image-select')
export class _ extends LitElement {

    static get styles() {
        return [css`
            :host {
                display: flex;
                flex-direction: column;
                row-gap: 18px;
                column-gap: 24px;
            }
            .top-row {
                grid-column: 1 / -1;
                display: flex;
                justify-content: space-between;
            }
            h2 {
                margin: 0;  font-family: Poppins;
                font-size: 18px;
                font-weight: normal;
            }
            .search-row {
                display: flex;
                align-items: center;
                column-gap: 24px;
            }
            .search-row ::slotted([slot=search-input]) {
                width: 100%;
            }
            .bottom-row {
                display: flex;
                justify-content: space-between;
                align-items: center;
            }
            .image-wrapper {
                grid-column: 1 / -1;
                display: grid;
                grid-template-columns: repeat(auto-fit, 118px);
                grid-auto-rows: 118px;
                gap: 15px;
                justify-content: space-between;
                margin-top: 16px;
            }
            ::slotted([slot=images]) {
                display: grid;
                place-content: center;
            }
        `];
    }

    @property({type: String})
    label?: string;

    render() {
        return html`
            <div class="top-row">
                <h2>${this.label || nothing}</h2>
                <slot name="hide-overlay"></slot>
            </div>
            <div class="search-row">
                <slot name="search-input"></slot>
                <slot name="filters"></slot>
            </div>
            <div class="bottom-row">
                <slot name="only-background-checkbox"></slot>
                <slot name="upload"></slot>
            </div>
            <div class="image-wrapper">
                <slot name="images"></slot>
            </div>
        `;
    }
}
