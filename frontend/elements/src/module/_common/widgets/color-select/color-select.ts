import { LitElement, html, css, customElement, property } from 'lit-element';

@customElement('color-select')
export class _ extends LitElement {

    static get styles() {
        return [css`
            h2 {
                margin: 0;
                font-size: 18px;
                color: var(--dark-gray-6);
                font-weight: 500;
                grid-column: 1 / -1;
            }
            :host {
                display: grid;
                grid-template-columns: 1fr auto 1fr;
                row-gap: 32px;
            }
            .sections {
                grid-column: 2;
                display: grid;
                row-gap: 32px;
            }
            hr {
                width: 100%;
                background: var(--light-gray-4);
                grid-column: 1 / -1;
            }
            ::slotted([slot=add-color]) {
                grid-column: 2;
            }
        `];
    }

    @property()
    label: string = "";

    render() {
        return html`
            <h2>${this.label}</h2>
            <div class="sections">
                <slot name="sections"></slot>
            </div>
            <hr>
            <slot name="add-color"></slot>
        `;
    }
}
