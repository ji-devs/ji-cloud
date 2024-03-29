import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/titles/ji";
import "@elements/core/titles/variants/underlined-title";
import "@elements/core/inputs/composed/search";
import { nothing } from "lit-html";

@customElement("category-page")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                .main-wrapper {
                    padding: 40px;
                }
                .wrapper {
                    display: flex;
                    padding-top: 40px;
                    border-bottom: solid 1px #e5e7ef;
                }

                .middle {
                    padding-left: 40px;
                    margin-right: 24px;
                }
                .slot-wrapper {
                    display: block;
                    margin-top: 18px;
                }
                ::slotted([slot="middle"]) {
                    display: flex;
                    flex-direction: column;
                    gap: 8px;
                    margin-top: 18px;
                    min-width: 848px;
                }
                .title-wrapper {
                    display: flex;
                    align-items: center;
                }
                .title-wrapper .title {
                    margin-right: 20px;
                }
                .sub-wrapper {
                    display: flex;
                    justify-content: space-between;
                }
            `,
        ];
    }

    render() {
        const STR_LABEL = "Edit Categories";
        const STR_CATEGORIES = "Categories";
        const STR_ADD = "Add Category";

        return html`
            <div class="main-wrapper">
                <underlined-title title=${STR_LABEL}></underlined-title>
                <div class="wrapper">
                    <div class="middle">
                        <div class="sub-wrapper">
                            <div class="title-wrapper">
                                <title-ji class="title" color="blue"
                                    >${STR_CATEGORIES}</title-ji
                                >
                                <slot name="expand"></slot>
                            </div>
                            <slot name="add"></slot>
                        </div>
                        <slot name="middle"></slot>
                    </div>
                </div>
            </div>
        `;
    }
}
