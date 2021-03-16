import { LitElement, html, css, customElement } from 'lit-element';

@customElement('button-collection')
export class _ extends LitElement {
    static get styles() {
        return [css`
            :host {
                display: flex;
                border: solid 1px var(--light-blue-5);
                border-radius: 14px;
                justify-content: space-evenly;
                padding: 14px 0;
            }
        `];
    }

    render() {
        return html`
            <slot></slot>
        `;
    }
}
