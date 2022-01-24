import { LitElement, html, css, customElement } from "lit-element";

const STR_HEADER = "Select theme";

@customElement("theme-selector")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                .options {
                    margin-top: 48px;
                    display: grid;
                    grid-template-columns: repeat(2, auto);
                    justify-content: space-between;
                    row-gap: 18px;
                }
                @media (min-width: 1920px) {
                    .options {
                        row-gap: 47px;
                    }
                }
                .top-line {
                    display: grid;
                    grid-template-columns: auto auto;
                    justify-content: space-between;
                }
                h2 {
                    margin: 0px;
                    font-family: Poppins;
                    font-weight: normal;
                    font-size: 16px;
                }
                @media (min-width: 1920px) {
                    h2 {
                        font-size: 18px;
                    }
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
