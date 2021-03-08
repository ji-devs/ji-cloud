import { LitElement, html, css, customElement, property } from "lit-element";

@customElement("locale-select-columns-item")
export class _ extends LitElement {

    static get styles() {
        return [css`
            :host {
                display: block;
                cursor: pointer;
            }
            :host([active]) {
                background-color: orange;
            }
        `]
    }

    @property({type: Boolean, reflect: true})
    active: boolean = false;

    render() {
        return html`
            <slot></slot>
        `;
    }
}
