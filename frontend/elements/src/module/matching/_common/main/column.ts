import { LitElement, html, css, customElement, property } from "lit-element";
import { classMap } from "lit-html/directives/class-map";
import { nothing } from "lit-html";

@customElement("matching-column")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: flex;
                    flex-direction: column;
                    gap: 16rem;
                }
            `,
        ];
    }

    render() {
        return html`<slot></slot>`;
    }
}
