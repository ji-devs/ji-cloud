import { LitElement, html, css, customElement, property } from "lit-element";
import { actionStyles } from "./action-styles";

export type Kind = "replay" | "exit" | "share" | "continue";

const STR_LABEL_LOOKUP: { [key in Kind]: string } = {
    "replay": "Replay",
    "exit": "Exit",
    "share": "Share",
    "continue": "Continue",
};

const icon: { [key in Kind]: string } = {
    "replay": "fa-solid fa-rotate-left",
    "exit": "fa-solid fa-arrow-right-from-bracket",
    "share": "fa-solid fa-share-nodes",
    "continue": "fa-solid fa-arrow-right",
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
                    justify-items: center;
                    row-gap: 6px;
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
