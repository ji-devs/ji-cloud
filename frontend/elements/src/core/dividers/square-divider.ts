import { MEDIA_UI } from "@utils/path";
import { LitElement, html, css, customElement, property } from "lit-element";
export type ColorBorder = "blue" | "none";
export type Size = "small" | "large";
@customElement("square-divider")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                input[type="number"] {
                    background-color: #ffffff;
                    border-radius: 14px;
                }
                .small {
                    width: 64px;
                    height: 64px;
                }
                .blue {
                    border: solid 1px #5590fc;
                }
            `,
        ];
    }

    @property()
    colorborder: ColorBorder = "blue";

    @property()
    size: Size = "small";

    render() {
        const { colorborder, size } = this;

        return html` <input type="number" class="${colorborder} ${size}" /> `;
    }
}
