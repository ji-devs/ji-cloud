import { LitElement, html, css, customElement, property, unsafeCSS } from "lit-element";
import {classMap} from "lit-html/directives/class-map";

const STR_LABEL = "Good job!";

@customElement("play-ending")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
            :host {
                display: flex;
                width: calc(1320rem * (1920/1719));
                height: 100%;
                align-items: center;
                justify-content: center;
            }

            .label {
                font-weight: 900;
                font-size: calc(56rem * (1920/1719));
                text-align: center;
                color: #fd7076;
            }
            `,
        ];
    }

    render() {

        return html`
            <div>
                <img-ui path="module/_common/play/ending.png"></img-ui>
                <div class="label">${STR_LABEL}</div>
            </div>`
    }
}
