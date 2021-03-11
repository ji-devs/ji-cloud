import { LitElement, html, css, customElement, property } from 'lit-element';

@customElement('color-select')
export class _ extends LitElement {

    static get styles() {
        return [css`
            :host {
                display: grid;
                grid-template-rows: 30px min-content 1px 30px;
                row-gap: 30px;
            }
            h2 {
                margin: 0;
            }
            .items {
                display: grid;
                grid-template-columns: repeat(6, auto);
                justify-content: space-around;
                grid-gap: 16px;
            }
            hr {
                width: 100%;
                background: var(--Light_Gray_4);
            }
            ::slotted([slot=add-color]) {
                justify-self: start;
            }
        `];
    }

    @property({type: String})
    label?: string;

    render() {
        return html`
            <h2>Select color</h2>
            <div class="items">
                <slot name="items"></slot>
            </div>
            <hr>
            <slot name="add-color"></slot>
        `;
    }
}
