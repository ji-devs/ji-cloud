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

// TS copy of ValueLabelTemplate
interface ValueLabelTemplate {
    prefix: string,
    postfix_singular: string,
    postfix_plural: string,
}

const STR_LABEL: Partial<Record<Kind, string>> = {
    attempts: "Player gets",
    "continue-some": "of",
    rounds: "Complete",
    n_choices: "Display",
    n_pairs: "Display",
    "n_pairs-alt": "Display",
    "cards-show-some": "Display",
};

const STR_LABEL_SUFFIX: Partial<Record<Kind, string[]>> = {
    attempts: ["try", "tries"],
    "time-limit": ["second", "seconds"],
    "continue-some": ["item", "items"],
    rounds: ["page", "pages"],
    n_choices: ["choice", "choices"],
    n_pairs: ["pair", "pairs"],
    "n_pairs-alt": ["pair", "pairs"],
    "cards-show-some": ["pair", "pairs"],
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
    private valueLabelTemplate?: ValueLabelTemplate;

    @property()
    value?: any;

    public set valueLabelTemplateFromString(template: string) {
        this.valueLabelTemplate = JSON.parse(template);
    }

    renderLabelSuffix() {
        if (this.valueLabelTemplate) {
            return html`<span>${
                this.value > 1
                ? this.valueLabelTemplate.postfix_plural
                : this.valueLabelTemplate.postfix_singular
            }</span>`;
        } else {
            const suffix = STR_LABEL_SUFFIX[this.kind];

            if (!suffix) {
                return nothing;
            }

            return html`<span>${this.value > 1 ? suffix[1] : suffix[0]}</span>`;
        }
    }

    render() {
        const { kind } = this;

        let label = null;
        if (this.valueLabelTemplate) {
            label = this.valueLabelTemplate.prefix;
        } else {
            label = STR_LABEL[kind];
        }

        return html`
            ${label ? html`<span>${label}</span>` : nothing}
            <slot></slot>
            ${this.renderLabelSuffix()}
        `;
    }
}
