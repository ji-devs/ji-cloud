import {
    LitElement,
    svg,
    html,
    css,
    customElement,
    property,
} from "lit-element";
import { classMap } from "lit-html/directives/class-map";
import { nothing } from "lit-html";
import { ThemeId } from "@elements/_themes/themes";
import { styleMap } from "lit-html/directives/style-map";
import { getEmptyStyle } from "@elements/module/_groups/cards/helpers";
import { Size, cardStyles } from "./styles";

export type Kind = "question" | "translucent";

@customElement("empty-card")
export class _ extends LitElement {
    static get styles() {
        return [
            ...cardStyles,
            css`
                .translucent {
                    border: solid 2rem var(--light-blue-4);
                    background-color: rgba(255, 255, 255, 0.8);
                }
                .question-mark {
                    border-style: dashed;
                }
                .question-mark > svg {
                    width: 61.78rem;
                    height: 97.75rem;
                }

                .question-mark-fill {
                    fill: #d3ddea;
                }
            `,
        ];
    }

    @property()
    theme: ThemeId = "blank";

    @property({ reflect: true })
    size: Size = "memory";

    @property({ reflect: true })
    kind: Kind = "translucent";

    @property({ type: Boolean, reflect: true })
    active: boolean = false;

    render() {
        const { theme, active, kind } = this;

        switch (kind) {
            case "question":
                return renderQuestion(theme, active);
            case "translucent":
                return renderTranslucent();
        }
    }
}

function renderQuestion(theme: ThemeId, active: boolean) {
    const style = getEmptyStyle(theme, active, 0.7);
    return html`
        <section>
            <div class="content question-mark" style=${style}>
                ${renderQuestionMark(theme, active)}
            </div>
        </section>
    `;
}

function renderTranslucent() {
    return html`
        <section>
            <div class="content translucent">&nbsp;</div>
        </section>
    `;
}

function renderQuestionMark(theme: ThemeId, active: boolean) {
    const style = active
        ? styleMap({ fill: `var(--theme-${theme}-cards-border-color)` })
        : styleMap({});

    return svg`
		<svg xmlns="http://www.w3.org/2000/svg" width="39.234" height="61.713" viewBox="0 0 39.234 61.713">
		<g id="noun_question_mark_2152791" data-name="noun_question mark_2152791" transform="translate(-186.501 -276.296)">
		<g id="Group_15074" data-name="Group 15074" transform="translate(101.617 276.287)">
		<path style=${style} class="question-mark-fill" data-name="Path 155363" d="M124.118,19.591a19.617,19.617,0,0,0-39.234,0h11.6a8.018,8.018,0,1,1,11.962,6.962v.007a19.677,19.677,0,0,0-8.451,9.914h0a19.541,19.541,0,0,0-1.128,4.489h.011v3.468h11.854V40.964h-.022a8.051,8.051,0,0,1,3.733-4.489h-.011A19.552,19.552,0,0,0,124.118,19.591Z" />
		<circle style=${style} class="question-mark-fill" data-name="Ellipse 613" cx="7" cy="7" r="7" transform="translate(99 47.722)" />
		</g>
		</g>
		</svg>
	`;
}
