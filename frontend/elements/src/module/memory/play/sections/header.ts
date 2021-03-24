import { LitElement, html, css, customElement, property, unsafeCSS } from "lit-element";
import { mediaUi } from "@utils/path";

const STR_TITLE = "Find the matching pairs";

@customElement("play-header")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: block;
                    margin-top: calc(32rem * (1920/1719));
                    margin-bottom: calc(34rem * (1920/1719));
                }
                .title {
                  font-size: calc(32rem * (1920/1719));
                  font-weight: 900;
                  letter-spacing: calc(-0.32rem * (1920/1719));
                  color: var(--main-yellow);
                }
            `,
        ];
    }

    render() {
        return html`<div class="title">${STR_TITLE}</div>`
    }
}
