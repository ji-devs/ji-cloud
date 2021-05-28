import { LitElement, html, css, customElement, property} from 'lit-element';
import { nothing } from "lit-html";

export type imageMode = 'image' | 'background';

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
            :host([imageMode=image]) {
                --image-width: 118px;
            }
            :host([imageMode=background]) {
                --image-width: 207px;
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
                display: grid;
                grid-template-columns: auto min-content;
                align-items: center;
            }
            .search-row ::slotted([slot=search-input]) {
                width: 100%;
            }
            .search-row ::slotted([slot=filters]) {
                grid-column: 2;
                margin-left: 24px;
            }
            .bottom-row {
                display: grid;
                grid-template-columns: min-content min-content;
                justify-content: space-between;
                align-items: center;
            }
            .bottom-row ::slotted([slot=upload]) {
                grid-column: 2;
            }
            .image-wrapper {
                grid-column: 1 / -1;
                display: grid;
                grid-template-columns: repeat(auto-fit, var(--image-width));
                grid-auto-rows: 118px;
                gap: 15px;
                justify-content: space-between;
                margin-top: 16px;
            }
            ::slotted([slot=images]) {
                display: grid;
                place-content: center;
                cursor: pointer;
            }
        `];
    }

    @property({type: String})
    label?: string;

    @property({type: String, reflect: true})
    imageMode: imageMode = "image";

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
