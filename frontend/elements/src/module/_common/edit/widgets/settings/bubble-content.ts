import {
    LitElement,
    html,
    css,
    customElement,
    property,
} from "lit-element";
import { nothing } from "lit-html";
import "@elements/core/images/ui";
import { Kind } from "./button";

const STR_LABEL: Partial<Record<Kind, string>> = {
    attempts: "Player gets",
    "continue-some": "of",
    rounds: "Play",
    n_choices: "Display",
    n_pairs: "Display",
};

const STR_LABEL_SUFFIX: Partial<Record<Kind, string>> = {
    attempts: "tries",
    "time-limit": "minutes",
    "continue-some": "items",
    rounds: "rounds",
    n_choices: "pairs",
    n_pairs: "pairs",
};

@customElement("module-settings-bubble-content")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: flex;
                    align-items: center;
                }

                ::slotted(*) {
                    margin-left: 8px;
                    margin-right: 8px;
                }

                ::slotted(input) {
                    width: 2em;
                }
            `,
        ];
    }

    @property()
    kind: Kind = "attempts";

    render() {
        const { kind } = this;

        const label = STR_LABEL[kind];
        const label_suffix = STR_LABEL_SUFFIX[kind];

        return html`
            ${label ? html`<span>${label}</span>` : nothing}
            <slot></slot>
            ${label_suffix ? html`<span>${label_suffix}</span>` : nothing}
        `;
    }
}
