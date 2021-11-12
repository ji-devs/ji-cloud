import { MEDIA_UI } from "@utils/path";
import { LitElement, html, css, customElement, property } from "lit-element";
@customElement("sub-title")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                p {
                }
                .normal {
                    font-size: 20px;
                    font-weight: 300;
                    line-height: 0.1;
                    margin-bottom: 32px;
                }

                .medium {
                    font-family: Poppins;
                    font-size: 32px;
                    font-weight: 300;
                    margin-bottom: 0px;
                }

                .large {
                    font-size: 40px;
                    font-weight: 300;
                }

                .pink {
                    color: #f2777f;
                }

                .black {
                    color: #383838;
                }
            `,
        ];
    }

    @property()
    title: string = "";
    @property()
    size: string = "";
    @property()
    color: string = "";

    render() {
        const { title, size, color } = this;

        return html`
            <p class="${size ? "large" : "normal"} ${color ? "pink" : "black"}">
                ${title}
            </p>
        `;
    }
}
