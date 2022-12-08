import { LitElement, html, css, customElement } from "lit-element";

const STR_HEADER = "Select a theme";

@customElement("theme-selector")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                .options {
                    margin-top: 14px;
                    display: grid;
                    grid-template-columns: repeat(2, auto);
                    justify-content: space-between;
                    row-gap: 14px;
                }
                .top-line {
                    display: grid;
                    grid-template-columns: auto auto;
                    justify-content: space-between;
                    align-items: center;
                    line-height: 30px;
                }
                h2 {
                    margin: 0px;
                    font-family: Poppins;
                    font-weight: normal;
                    font-size: 14px;
                }
            `,
        ];
    }

    render() {
        return html`
            <div class="top-line">
                <h2>${STR_HEADER}</h2>
                <slot name="action"></slot>
            </div>
            <div class="options">
                <slot></slot>
            </div>
        `;
    }
}
