import { LitElement, html, css, customElement, property } from "lit-element";
import { actionStyles } from "./action-styles";

export type Kind = "back" | "forward";

@customElement("jig-play-move-button")
export class _ extends LitElement {
    static get styles() {
        return [
            actionStyles,
            css`
                :host([kind="back"]) img-ui {
                    transform: rotate(180deg);
                }
            `,
        ];
    }

    @property({ reflect: true })
    kind: Kind = "back";

    render() {
        return html`
            <button class="action ${this.kind === "back" ? "small" : "large"}">
                <img-ui path="entry/jig/play/arrow-right.svg"></img-ui>
            </button>
        `;
    }
}
