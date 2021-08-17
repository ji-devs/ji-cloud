import { LitElement, html, css, customElement, property } from 'lit-element';

@customElement('pill-close-delete')
export class _ extends LitElement {
    static get styles() {
        return [css`
            button {
                border: 0;
                background-color: var(--main-blue);
                padding: 0;
                color: white;
                width: 16px;
                height: 16px;
                font-size: 16px;
                border-radius: 50%;
                cursor: pointer;
            }
        `];
    }

    render() {
        return html`
            <button>&times;</button>
        `;
    }
}
