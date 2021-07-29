import { LitElement, html, css, customElement, property } from "lit-element";
import { actionStyles } from "./action-styles";

export type Mode = "play" | "pause";

@customElement("jig-play-play-pause")
export class _ extends LitElement {
    static get styles() {
        return [
            actionStyles,
            css`
            `,
        ];
    }

    @property()
    mode: Mode = "play";

    render() {
        return html`
            <button class="action large">
                <img-ui path="entry/jig/play/${this.mode}.svg"></img-ui>
            </button>
        `;
    }
}
