import { LitElement, html, css, customElement, property } from "lit-element";
import { classMap } from "lit-html/directives/class-map";

@customElement("circle-div")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                div {
                    border-radius: 50%;
                    margin-right: 24px;
                }
                .yellow {
                    background-color: #fed657;
                }

                .white {
                    background-color: #ffffff;
                }

                .pink {
                    background-color: #fd6b71;
                }

                .small {
                    width: 12px;
                    height: 12px;
                }

                .medium {
                    width: 16px;
                    height: 16px;
                }
            `,
        ];
    }

    @property()
    size: string = "";

    @property()
    color: string = "";

    render() {
        const { size, color } = this;

        return html` <div class="${size} ${color}"></div> `;
    }
}
