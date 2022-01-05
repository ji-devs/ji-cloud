import {
    LitElement,
    html,
    css,
    customElement,
    property,
    unsafeCSS,
    state,
} from "lit-element";
import "@elements/core/images/ui";
import { mediaUi } from "@utils/path";
import { classMap } from "lit-html/directives/class-map";
import { nothing } from "lit-html";
import { JigFocus } from "@elements/module/_common/types";

const STR_JIG = "JIG";
const STR_RESOURCE = "Resource";

const STR_CREATE_JIG = "Create a New ";
const STR_TEMPLATE_PARAGRAPH_1 = "We have created lesson plans you can use for teaching. Create ";
const STR_TEMPLATE_PARAGRAPH_2 = " from one of these templates to easily address all your student’s learning needs!";
const STR_RECENT_1 = "My Recent ";
const STR_RECENT_2 = "s";
const STR_SEE_ALL_TEMPLATES = "See all templates";
const STR_SEE_LESS_TEMPLATES = "See less templates";

@customElement("jig-gallery")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: grid;
                    min-height: 100vh;
                    grid-template-rows: 0px auto 0px 1fr;
                }
                .width-holder {
                    max-width: 1720px;
                    margin: 0 auto;
                }
                .novel-row-1 {
                    z-index: 1;
                    pointer-events: none;
                }
                .novel-row-1 .width-holder {
                    padding-left: 698px;
                }
                .novel-img-1 {
                    height: 125px;
                    width: 125px;
                }
                .top-section {
                    background-color: var(--light-blue-3);
                }
                .top-section .width-holder {
                    grid-column: 1 / -1;
                    padding: 100px;
                    display: grid;
                    grid-template-columns: auto 1fr;
                    justify-content: space-between;
                }
                .create-jig-header {
                    font-size: 56px;
                    font-weight: 900;
                    color: var(--orange);
                    margin: 0;
                }
                .template-paragraph {
                    font-size: 18px;
                    font-weight: 300;
                    grid-column: 1;
                    margin: 0;
                    max-width: 540px;
                }
                .new-jig-section {
                    display: grid;
                    grid-template-columns: repeat(auto-fit, 232px);
                    gap: 32px;
                    justify-content: end;
                }
                .new-jig-section-width-holder {
                    grid-column: 2;
                    display: grid;
                    grid-template-rows: auto auto;
                    row-gap: 56px;
                }
                .new-jig-items-wrapper {
                    max-height: 185px;
                    overflow-y: hidden;
                    padding: 10px;
                    width: 100%;
                    display: grid;
                    grid-template-columns: inherit;
                    gap: inherit;
                    grid-column: 1 / -1;
                }
                .all-templates-visible .new-jig-items-wrapper {
                    max-height: initial;
                }
                .new-jig-items {
                    grid-column: 1 / -1;
                    justify-content: flex-end;
                    display: grid;
                    grid-template-columns: inherit;
                    gap: inherit;
                    grid-column: 1 / -1;
                }
                .see-all-templates-button {
                    grid-column: 1 / -1;
                    text-align: center;
                    padding-left: 10px; /* bad */
                }
                .see-all-templates-button span {
                    display: flex;
                    justify-content: center;
                    gap: 8px;
                }
                .see-all-templates-button img-ui {
                    height: 22px;
                    transition: transform 0.3s;
                }
                .all-templates-visible .see-all-templates-button img-ui {
                    transform: rotate(180deg);
                }
                .novel-row-2 {
                    z-index: 1;
                    margin-top: -140px;
                    pointer-events: none;
                }
                .novel-row-2 .width-holder {
                    padding-left: 473px;
                }
                .novel-img-2 {
                    height: 225px;
                    width: 225px;
                }
                .novel-img-3 {
                    height: 145px;
                    width: 145px;
                }
                .bottom-section {
                    grid-column: 1 / -1;
                    background-image: url(${unsafeCSS(
                        mediaUi("entry/jig/gallery/background.png")
                    )});
                    background-size: cover;
                }
                .bottom-section .width-holder {
                    padding: 100px;
                    display: grid;
                    row-gap: 48px;
                }
                .recent-top-line {
                    display: grid;
                    grid-template-columns: auto auto 224px;
                    column-gap: 32px;
                    align-items: center;
                }
                .recent-header {
                    color: var(--dark-blue-4);
                    font-size: 40px;
                    font-weight: 800;
                    margin: 0;
                }
                ::slotted([slot="filters"]) {
                    justify-self: end;
                    min-width: 300px;
                    --background-color: #ffffff;
                }
                .recent-items {
                    display: grid;
                    grid-template-columns: repeat(auto-fill, 230px);
                    gap: 34px;
                    justify-content: space-between;
                }
            `,
        ];
    }

    @property()
    title: string = "";

    @property()
    jigFocus: JigFocus = "modules";

    @state()
    private allTemplatesVisible = false;

    private focusString() {
        if (this.jigFocus === "resources") {
            return STR_RESOURCE;
        } else {
            return STR_JIG;
        }
    }

    render() {
        return html`
            <div class="novel-row-1">
                <div class="width-holder">
                    <img-ui
                        class="novel-img-1"
                        path="entry/jig/gallery/novel-img-1.png"
                    ></img-ui>
                </div>
            </div>
            <section class="top-section">
                <div class="width-holder">
                    <div class="text-side">
                        <h1 class="create-jig-header">
                            ${STR_CREATE_JIG + this.focusString()}
                        </h1>
                        <p class="template-paragraph">
                            ${STR_TEMPLATE_PARAGRAPH_1}
                            ${this.focusString()}
                            ${STR_TEMPLATE_PARAGRAPH_2}
                        </p>
                    </div>
                    <div
                        class="new-jig-section ${classMap({
                            "all-templates-visible": this.allTemplatesVisible,
                        })}"
                    >
                        <div class="new-jig-items-wrapper">
                            <div class="new-jig-items">
                                <slot name="create-jig"></slot>
                                <slot name="jig-templates"></slot>
                            </div>
                        </div>
                        ${
                            this.jigFocus === "resources" ? nothing : html`
                                <div class="see-all-templates-button">
                                    <button-rect
                                        kind="text"
                                        color="blue"
                                        weight="bold"
                                        @click="${() =>
                                            (this.allTemplatesVisible =
                                                !this.allTemplatesVisible)}"
                                    >
                                        <span>
                                            ${this.allTemplatesVisible
                                                ? STR_SEE_LESS_TEMPLATES
                                                : STR_SEE_ALL_TEMPLATES}
                                            <img-ui
                                                path="core/_common/chevron-down-blue.svg"
                                            ></img-ui>
                                        </span>
                                    </button-rect>
                                </div>
                            `
                        }
                    </div>
                </div>
            </section>
            <div class="novel-row-2">
                <div class="width-holder">
                    <img-ui
                        class="novel-img-2"
                        path="entry/jig/gallery/novel-img-2.png"
                    ></img-ui>
                    <img-ui
                        class="novel-img-3"
                        path="entry/jig/gallery/novel-img-3.png"
                    ></img-ui>
                </div>
            </div>
            <section class="bottom-section">
                <div class="width-holder">
                    <div class="recent-top-line">
                        <h2 class="recent-header">
                            ${STR_RECENT_1}
                            ${this.focusString()}
                            ${STR_RECENT_2}
                        </h2>
                        <slot class="filters" name="filters"></slot>
                        <slot class="search-input" name="search-input"></slot>
                    </div>
                    <div class="recent-items">
                        <slot name="recent-items"></slot>
                    </div>
                </div>
            </section>
        `;
    }
}
