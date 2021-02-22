import { LitElement, html, css, customElement } from "lit-element";

@customElement("locale-cell")
export class _ extends LitElement {

    static get styles() {
        return [css`
            :host {
                margin: 0;
                padding: 0;
                display: inline-block;
                border: solid white 2px;
                background-color: var(--row-background-color);
            }
            ::slotted(:not(input[type=checkbox])) {
                background-color: transparent;
                border: 0;
                height: 100%;
                padding: 0 10px;
                min-width: 100px;
                width: calc(100% - 20px);
            }
            /* ::slotted(textarea) {
                resize: vertical;
                min-width: 300px;
                min-height: 100px;
            } */
        `]
    }

    render() {
        return html`
            <slot></slot>
        `;
    }
}
