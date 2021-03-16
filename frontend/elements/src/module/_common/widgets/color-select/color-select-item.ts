import { LitElement, html, css, customElement, property} from 'lit-element';

@customElement('color-select-item')
export class _ extends LitElement {

    static get styles() {
        return [css`
            :host {
                height: 56px;
                width: 56px;
                border-radius: 50%;
                display: inline-grid;
                padding: 3px;
            }
            :host([selected]) {
                box-shadow: var(--main-blue) 0px 0px 0pt 3px;
            }
            :host::before {
                grid-row: 1;
                grid-column: 1;
                content: '';
                height: 100%;
                width: 1px;
                display: block;
                margin: auto;
                background: var(--light-gray-4);
                transform: rotate(45deg);
            }
            .color-item {
                grid-row: 1;
                grid-column: 1;
                z-index: 1;
                border-radius: 50%;
                border: solid 1px var(--light-gray-4);
            }
            .color-item:not([disabled]) {
                cursor: pointer;
            }
        `];
    }

    @property({type: String})
    color?: string;

    @property({type: Boolean, reflect: true})
    selected: boolean = false;

    @property({type: Boolean, reflect: true})
    disabled: boolean = false;

    render() {
        return html`
            <div
                class="color-item"
                style="background-color: ${!this.disabled ? this.color : "#fff"}"
            ></div>
        `;
    }
}
