import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/buttons/fa-button";

@customElement("theme-custom-background")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: grid;
                    padding: 12px 0;
                    height: 100%;
                    grid-template-rows: minmax(0px, 1fr);
                    box-sizing: border-box;
                }
                .main {
                    border-radius: 16px;
                    box-shadow: 0 3px 16px 0 rgba(0, 0, 0, 0.25);
                    padding-top: 16px;
                    grid-area: 1 / 1;
                    display: grid;
                    grid-template-rows: min-content minmax(0px, 1fr);
                    row-gap: 24px;
                    scrollbar-width: thin;
                    scrollbar-color: #e7f0fe transparent;
                }
                :host([tabbed]) .main {
                    overflow: hidden;
                }
                :host(:not([tabbed])) .main {
                    padding: 12px;
                    overflow: auto;
                }
                .main::-webkit-scrollbar-track {
                    background-color: transparent;
                }
                .main::-webkit-scrollbar {
                    width: 8px;
                }
                .main::-webkit-scrollbar-thumb {
                    border-radius: 4px;
                    background-color: #c1c1c1;
                }
                .main::-webkit-scrollbar-button {
                    background-color: transparent;
                    height: 8px;
                }
                h2 {
                    font-size: 24px;
                    font-weight: 600;
                    color: #fd7076;
                    margin: 0;
                }
                :host([tabbed]) h2 {
                    margin: 0 12px;
                }
                ::slotted([slot=close]) {
                    grid-area: 1 / 1;
                    justify-self: end;
                    align-self: start;
                    margin-top: 16px;
                    margin-right: 16px;
                    z-index: 1;
                }
            `,
        ];
    }

    @property({ type: Boolean, reflect: true })
    tabbed: boolean = false;

    @property()
    header = "";

    render() {
        return html`
            <div class="main">
                <h2>${this.header}</h2>
                <slot></slot>
            </div>
            <slot name="close"></slot>
        `;
    }
}
