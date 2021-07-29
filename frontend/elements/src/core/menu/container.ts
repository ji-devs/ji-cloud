import { LitElement, html, css, customElement, property } from 'lit-element';
import { classMap } from "lit-html/directives/class-map";

@customElement('menu-container')
export class _ extends LitElement {
    static get styles() {
        return [css`
            section {
                border-radius: 8px;
                -webkit-backdrop-filter: blur(30px);
                backdrop-filter: blur(30px);
                box-shadow: 0 3px 16px 0 rgba(0, 0, 0, 0.2);
                background-color: var(--white);
                display: inline-block;
                padding: 14px 33px 16px 14px;
            }
        `];
    }


    render() {
        return html`<section><slot></slot></section>`;
    }
}
