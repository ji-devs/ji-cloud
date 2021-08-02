import { LitElement, html, css, customElement, property } from "lit-element";
import { actionStyles } from "./action-styles";

@customElement("jig-play-replay")
export class _ extends LitElement {
    static get styles() {
        return [
            actionStyles,
            css`
                .action img-ui {
                    width: 60px;
                }
            `,
        ];
    }

    render() {
        return html`
            <button class="action huge">
                <img-ui path="entry/jig/play/replay.svg"></img-ui>
            </button>
        `;
    }
}
