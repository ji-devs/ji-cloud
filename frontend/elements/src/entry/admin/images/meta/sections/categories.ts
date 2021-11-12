import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/titles/ji";
import { nothing } from "lit-html";

const STR_CATEGORIES_SELECT = "Categories";
const STR_CATEGORIES_REPORT = "Categories summary";

@customElement("image-meta-section-categories")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: grid;
                    grid-template-columns: 1fr 1fr;
                    gap: 10px;
                }
                .category-select,
                .category-report {
                    padding: 10px;
                    /* Just a quick fix to make it look ok-ish*/
                    min-width: calc(100% / 2.5);
                }
                .category-report {
                    border-radius: 10px;
                    background-color: #edf2ff;
                }

                header {
                    font-size: 16px;
                    font-weight: 500;
                    font-stretch: normal;
                    font-style: normal;
                    line-height: 1.25;
                    letter-spacing: -0.16px;
                    text-align: left;
                    color: #5590fc;
                    margin-bottom: 10px;
                }
            `,
        ];
    }

    render() {
        return html`
            <div class="category-select">
                <div style="display: flex">
                    <header>${STR_CATEGORIES_SELECT}</header>
                    <slot name="expand"></slot>
                </div>
                <slot name="category-select"></slot>
            </div>
            <div>
                <header>${STR_CATEGORIES_REPORT}</header>
                <card-blue>
                    <slot name="category-report"></slot>
                </card-blue>
            </div>
        `;
    }
}
