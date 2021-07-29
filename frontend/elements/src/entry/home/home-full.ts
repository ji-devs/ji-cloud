import { LitElement, html, css, customElement, property } from 'lit-element';

@customElement('home-full')
export class _ extends LitElement {
    static get styles() {
        /* REMOVE ME - JUST ADDED QUICKLY TO TRIGGER CI */
        return [css`
            
        `];
    }

    render() {
        return html`
            <main>
                <slot></slot>
            </main>
        `;
    }
}
