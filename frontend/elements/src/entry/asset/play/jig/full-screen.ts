import { LitElement, html, css, customElement, property } from "lit-element";
import { actionStyles } from "./action-styles";
import "@elements/core/buttons/fa-button";

export type Mode = "play" | "pause";

@customElement("jig-play-full-screen")
export class _ extends LitElement {
    static get styles() {
        return [actionStyles, css``];
    }

    @property({ type: Boolean })
    isFullScreen: boolean = false;

    render() {
        return html`
            <fa-button
                class="action middle"
                icon="fa-regular fa-arrows-${this.isFullScreen ? 'min' : 'max'}imize"
            ></fa-button>
        `;
    }
}
