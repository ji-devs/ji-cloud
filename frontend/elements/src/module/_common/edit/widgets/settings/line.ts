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
    | "video-features"
    | "ordering";

const STR_LABEL: Record<Kind, string> = {
    "card-view": "How should your cards be displayed?",
    "game-display": "How should your cards be displayed?",
    "rounds": "How many pages should student complete?",
    "hint": "Highlight clickable areas:",
    "next": "Student views this page until...",
    "time-limit": "Would you like to set a time limit?",
    "attempts": "How many tries does the student get?",
    "score": "Would you like to include score?",
    "video-play": "How to play your video:",
    "video-features": "Play features",
    "ordering": "Would you like questions to be asked in order?",
};

@customElement("module-settings-line")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: grid;
                    grid-template-columns: 90px 1fr;
                    column-gap: 18px;
                }

                .label {
                    margin-top: 11px;
                    font-weight: 500;
                    line-height: 20px;
                    text-align: left;
                    color: var(--dark-gray-4);
                    font-size: 13px;
                }

                .options {
                    display: grid;
                    grid-template-columns: 1fr 1fr;
                    gap: 12px;
                }
            `,
        ];
    }

    @property()
    kind?: Kind = "card-view";

    @property({ type: String })
    label?: String;

    @property({ type: Boolean })
    borderTop: boolean = false;

    render() {
        return html`
            <div class="label">${this.getLabel()}</div>
            <div class="options"><slot></slot></div>
        `;
    }

    getLabel() {
        const { kind, label } = this;

        if (label) {
            return label;
        } else if (kind) {
            return STR_LABEL[kind];
        } else {
            return undefined;
        }
    }
}
