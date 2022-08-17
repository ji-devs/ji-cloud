import { LitElement, html, css, customElement, property } from "lit-element";
import { actionStyles } from "./action-styles";

export type Kind = "replay" | "exit" | "share";

const STR_LABEL_LOOKUP: { [key in Kind]: string } = {
    "replay": "Replay",
    "exit": "Exit",
    "share": "Share",
};

const icon: { [key in Kind]: string } = {
    "replay": "fa-solid fa-rotate-left",
    "exit": "fa-solid fa-arrow-right-from-bracket",
    "share": "fa-solid fa-share-nodes",
};

@customElement("jig-play-done-action")
export class _ extends LitElement {
    static get styles() {
        return [
            actionStyles,
            css`
                button {
                    background-color: transparent;
                    border: 0;
                    display: grid;
                    row-gap: 12px;
                }
                fa-icon {
                    color: #fff;
                }
                .text {
                    color: var(--dark-gray-6);
                    font-size: 14px;
                    font-weight: 600;
                }
            `,
        ];
    }

    @property()
    kind: Kind = "replay";

    render() {
        return html`
            <button>
                <div class="action large">
                    <fa-icon icon=${icon[this.kind]}></fa-icon>
                </div>
                <span class="text">${STR_LABEL_LOOKUP[this.kind]}</span>
            </button>
        `;
    }
}
