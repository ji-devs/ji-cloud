import { LitElement, html, css, customElement, property } from "lit-element";

@customElement("hebrew-inputs-iframe")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                iframe {
                    height: 665px;
                    border: 0;
                    background-color: #ffffff;
                    border-radius: 16px;
                    box-shadow: rgb(0 0 0 / 25%) 0px 3px 16px 0px;
                }
            `,
        ];
    }

    @property({ type: Number })
    width: number = 100;

    @property()
    src: string = "";

    render() {
        return html`
            <style>
                iframe {
                    width: ${this.width}px;
                }
            </style>
            <iframe src="${this.src}"></iframe>
        `;
    }
}
