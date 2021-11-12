import { LitElement, html, css, customElement, property } from "lit-element";
import { classMap } from "lit-html/directives/class-map";

@customElement("stripe-along")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                div {
                    width: 1px;
                    height: 250px;
                    background-color: #ffffff;
                    opacity: 0.35;
                }
            `,
        ];
    }

    render() {
        const {} = this;

        return html` <div></div> `;
    }
}
