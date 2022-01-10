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
    rounds: "Complete",
    n_choices: "Display",
    n_pairs: "Display",
};

const STR_LABEL_SUFFIX: Partial<Record<Kind, string[]>> = {
    attempts: ["try", "tries"],
    "time-limit": ["minute", "minutes"],
    "continue-some": ["item", "items"],
    rounds: ["page", "pages"],
    n_choices: ["card", "cards"],
    n_pairs: ["pair", "pairs"],
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

    @property()
    value?: any;

    renderLabelSuffix() {
        const suffix = STR_LABEL_SUFFIX[this.kind];

        if (!suffix) {
            return nothing;
        }

        return html`<span>${this.value > 1 ? suffix[1] : suffix[0]}</span>`;
    }

    render() {
        const { kind } = this;

        const label = STR_LABEL[kind];
        const label_suffix = STR_LABEL_SUFFIX[kind];

        return html`
            ${label ? html`<span>${label}</span>` : nothing}
            <slot></slot>
            ${this.renderLabelSuffix()}
        `;
    }
}
