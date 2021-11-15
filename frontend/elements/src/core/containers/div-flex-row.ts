import { MEDIA_UI } from "@utils/path";
import { LitElement, html, css, customElement, property } from "lit-element";

@customElement("div-flex-row")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                div {
                    display: flex;
                }
            `,
        ];
    }

    render() {
        return html`
            <div>
                <slot></slot>
            </div>
        `;
    }
}
