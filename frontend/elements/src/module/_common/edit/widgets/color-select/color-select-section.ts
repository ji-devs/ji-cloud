import { LitElement, html, css, customElement, property} from 'lit-element';

@customElement('color-select-section')
export class _ extends LitElement {

    static get styles() {
        return [css`
            :host {
                display: grid;
                row-gap: 24px;
            }
            h3 {
                margin: 0;
                font-size: 16px;
                color: #4a4a4a;
                font-weight: 500;
            }
            .items {
                display: grid;
                grid-template-columns: repeat(6, 62px);
                gap: 24px;
            }
        `];
    }

    @property()
    label: string = "";

    render() {
        return html`
            <h3>${this.label}</h3>
            <div class="items">
                <slot name="items"></slot>
            </div>
        `;
    }
}
