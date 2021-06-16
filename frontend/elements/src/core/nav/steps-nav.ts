import { LitElement, html, css, customElement } from 'lit-element';

@customElement('steps-nav')
export class _ extends LitElement {

    static get styles() {
        return [css`
            :host {
                display: flex;
                justify-content: space-between;
            }
        `];
    }

    render() {
        return html`
            <slot></slot>
        `;
    }
}
