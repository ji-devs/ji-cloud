import {
    LitElement,
    html,
    css,
    customElement,
    property,
} from "lit-element";
import "@elements/core/images/ui";

export type Kind =
    | "card-view"
    | "game-display"
    | "rounds"
    | "hint"
    | "next"
    | "time-limit"
    | "attempts"
    | "score"
    | "video-play"
    | "video-features";

const STR_LABEL: Record<Kind, string> = {
    "card-view": "How should your cards be displayed?",
    "game-display": "How should your cards be displayed?",
    "rounds": "How many pages should student complete?",
    "hint": "Highlight clickable areas:",
    "next": "Student finishes this activity by...",
    "time-limit": "Would you like to set a time limit?",
    "attempts": "How many tries does the student get?",
    "score": "Would you like to include score?",
    "video-play": "How to play your video:",
    "video-features": "Play features",
};

@customElement("module-settings-line")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: grid;
                    grid-template-columns: 116px 1fr;
                    gap: 0px 32px;
                }
                @media (min-width: 1920px) {
                    :host {
                        grid-template-columns: 123px 1fr;
                        gap: 0px 70px;
                    }
                }

                .label {
                    margin-top: 11px;
                    font-weight: 500;
                    line-height: 20px;
                    text-align: left;
                    color: var(--dark-gray-4);
                    font-size: 14px;
                }
                @media (min-width: 1920px) {
                    .label {
                        font-size: 16px;
                    }
                }

                .options {
                    display: grid;
                    grid-template-columns: 64px 64px;
                    gap: 24px 52px;
                }
            `,
        ];
    }

    @property()
    kind: Kind = "card-view";

    @property({ type: Boolean })
    borderTop: boolean = false;

    render() {
        const { kind } = this;

        const label = STR_LABEL[kind];

        return html`
            <div class="label">${label}</div>
            <div class="options"><slot></slot></div>
        `;
    }
}
