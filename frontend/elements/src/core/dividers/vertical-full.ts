import { LitElement, html, css, customElement, property } from "lit-element";

@customElement("vertical-full")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                div {
                    width: 100%;
                    height: 1px;
                }
            `,
        ];
    }

    render() {
        return html` <div></div> `;
    }
}
