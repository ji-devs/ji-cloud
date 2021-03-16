import { LitElement, html, css, customElement } from 'lit-element';

@customElement('button-collection-item')
export class _ extends LitElement {
    static get styles() {
        return [css`
            :host {
                font-size: 18px;
                height: 36px;
                width: 36px;
                line-height: 36px;
                text-align: center;
                font-family: 'Poppins', sans-serif;
                color: var(--dark-gray-6);
                display: flex;
                font-weight: 500;
            }
            button {
                border: 0;
                background-color: transparent;
                padding: 0;
                height: inherit;
                width: inherit;
                font-size: inherit;
                line-height: inherit;
                font-family: inherit;
                font-weight: inherit;
                cursor: pointer;
            }
        `];
    }

    render() {
        return html`
            <button><slot></slot></button>
        `;
    }
}
