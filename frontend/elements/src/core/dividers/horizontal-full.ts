import { LitElement, html, css, customElement, property } from "lit-element";
export type Color = "black" | "blue";
@customElement("horizontal-full")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                div {
                    height: 100%;
                    width: 2px;
                    background-color: #707070;
                    margin-left: 14px;
                    margin-right: 14px;
                }
                .black {
                    background-color: #e5e7ef;
                }
                .blue {
                    background-color: #5590fc;
                }
            `,
        ];
    }

    @property()
    color: Color = "black";

    render() {
        const { color } = this;

        return html` <div class="${color}"></div> `;
    }
}
