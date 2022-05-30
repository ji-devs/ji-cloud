import { LitElement, html, css, customElement, property } from "lit-element";
import { actionStyles } from "./action-styles";

export type Kind = "replay" | "exit" | "share";

@customElement("jig-play-done-action")
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

    @property()
    kind: Kind = "replay";

    render() {
        return html`
            <button class="action huge">
                <img-ui path="entry/jig/play/${this.kind}.svg"></img-ui>
            </button>
        `;
    }
}
