import { LitElement, html, css, customElement, property } from "lit-element";
import { nothing } from "lit-html";

export class BgBlue extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    background-color: #e6f0ff;
                }
            `,
        ];
    }
}
