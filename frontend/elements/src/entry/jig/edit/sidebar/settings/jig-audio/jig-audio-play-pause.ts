import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/images/ui";

export type Mode = "play" | "pause";

@customElement("jig-audio-play-pause")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    cursor: pointer;
                }
                img-ui {
                    display: none;
                }
                :host([mode=play]) .play, :host([mode=pause]) .pause {
                    display: inline-block;
                }
            `,
        ];
    }

    @property({ reflect: true })
    mode: Mode = "pause";

    render() {
        return html`
            <img-ui class="play" path="entry/jig/settings/play.svg"></img-ui>
            <img-ui class="pause" path="entry/jig/settings/pause.svg"></img-ui>
        `;
    }
}
