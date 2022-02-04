import { LitElement, html, css, customElement } from "lit-element";
import "@elements/core/buttons/fa-button";

const STR_HEADER = "Design from scratch";

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
                    overflow: hidden;
                }
                h2 {
                    font-size: 24px;
                    font-weight: 600;
                    color: #fd7076;
                    margin: 0 12px;
                }
                ::slotted([slot=close]) {
                    grid-area: 1 / 1;
                    justify-self: end;
                    margin-top: 16px;
                    margin-right: 16px;
                }
            `,
        ];
    }

    render() {
        return html`
            <div class="main">
                <h2>${STR_HEADER}</h2>
                <slot></slot>
            </div>
            <slot name="close"></slot>
        `;
    }
}
