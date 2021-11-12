import { LitElement, html, css, customElement, property } from "lit-element";
import { classMap } from "lit-html/directives/class-map";

@customElement("spacer-fourty")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                div {
                    height: 40px;
                    width: 100%;
                    display: block;
                }
            `,
        ];
    }

    render() {
        const {} = this;

        return html` <div></div> `;
    }
}
