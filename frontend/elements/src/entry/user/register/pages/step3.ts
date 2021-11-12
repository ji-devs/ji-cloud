import { mediaUi } from "@utils/path";
import {
    LitElement,
    html,
    css,
    customElement,
    property,
    unsafeCSS,
} from "lit-element";
import "@elements/core/lists/list-horizontal";
import "@elements/core/lists/list-vertical";
import "@elements/core/titles/ji";
import "@elements/entry/user/_common/base-page";

const STR_TITLE = "Sign Up - Step 3";
const STR_SUBTITLE =
    "We want to provide content that is tailored to your needs and interests.";
const STR_SUBSUBTITLE =
    "Select as many as you like now. You can always edit this later under Teaching Settings in your profile page.";
const STR_AGES_LABEL = "I'm interested in content for this age group:";
const STR_AFFILIATIONS_LABEL = "I want to see content for these affiliations:";
const STR_SUBJECTS_LABEL = "I'm interested in teaching these subjects:";

@customElement("page-register-step3")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                .grid {
                    display: grid;
                    grid-template-columns: 1fr 1fr;
                    grid-template-rows: 1fr 1fr;
                    gap: 32px 31px;
                    grid-auto-flow: row;
                    grid-template-areas:
                        "tl tr"
                        "bottom bottom";
                }

                .ages {
                    grid-area: tl;
                }
                .subjects {
                    grid-area: tr;
                }
                .affiliations {
                    grid-area: bottom;
                }

                .card-grey {
                    padding: 32px 32px 32px 32px;
                    border-radius: 14px;
                    background-color: #f7f7f7;
                }
                h1 {
                    font-size: 32px;
                    font-weight: 900;
                    color: #5662a3;
                }

                .subtitle {
                    font-weight: 500;
                    color: var(--dark-gray-6);
                }

                .submit {
                    align-self: flex-start;
                }
            `,
        ];
    }

    render() {
        return html`
            <base-page>
                <h1>${STR_TITLE}</h1>

                <h4 class="subtitle">
                    ${STR_SUBTITLE}<br />${STR_SUBSUBTITLE}
                </h4>

                <div class="grid">
                    <div class="ages card-grey">
                        <list-vertical label="${STR_AGES_LABEL}">
                            <slot name="ages"></slot>
                        </list-vertical>
                    </div>
                    <div class="subjects card-grey">
                        <list-vertical label="${STR_SUBJECTS_LABEL}">
                            <slot name="subjects"></slot>
                        </list-vertical>
                    </div>
                    <div class="affiliations card-grey">
                        <list-vertical label="${STR_AFFILIATIONS_LABEL}">
                            <slot name="affiliations"></slot>
                        </list-vertical>
                    </div>
                </div>
                <div class="submit">
                    <slot name="submit"></slot>
                </div>
            </base-page>
        `;
    }
}
