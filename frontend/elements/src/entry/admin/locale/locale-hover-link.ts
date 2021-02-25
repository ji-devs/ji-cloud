import { LitElement, html, css, customElement, property } from "lit-element";
import { ifDefined } from "lit-html/directives/if-defined";

@customElement("locale-hover-link")
export class _ extends LitElement {

    static get styles() {
        return [css`
            :host(:hover) a {
                display: block;
            }
            a {
                display: none;
                position: absolute;
                background-color: white;
                line-height: 2em;
                margin-top: -1.5em;
                padding: 0 10px;
            }
            ::slotted(input) {
                background-color: transparent;
                border: 0;
                height: 100%;
                padding: 0 10px;
            }
        `]
    }

    @property()
    link?: string;

    render() {
        return html`
            <a href="${ifDefined(this.link)}">${this.link}</a>
            <slot></slot>
        `;
    }
}
