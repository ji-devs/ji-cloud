import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/images/ui";
import { homeStyles } from "../styles";

export type Mode = "home" | "results";

const STR_LEARNING = "Learning Through";
const STR_CREATION = "Creation";
const STR_MAKE_LEARNING = "Make learning awesome with";
const STR_JIGS = "JIGs";

const numberFormat = new Intl.NumberFormat().format;

@customElement("home-search-section")
export class _ extends LitElement {
    static get styles() {
        return [
            homeStyles,
            css`
                :host {
                    --padding: 88px;

                    display: block;
                    background-color: var(--light-blue-6);
                    padding: var(--padding) 0;
                }
                :host([mode="results"]) {
                    padding: calc(var(--padding) / 2) 0;
                }
                :host([mode="results"]) .home-only,
                :host([mode="home"]) .results-only {
                    opacity: 0;
                    pointer-events: none;
                }
                .width-holder {
                    display: grid;
                    grid-template-columns: auto;
                    justify-content: center;
                    align-items: center;
                }
                :host([mode="results"]) .width-holder {
                    justify-content: space-between;
                }
                .center-1 {
                    grid-column: 1 / -1;
                    grid-row: 1;
                }
                .center-2 {
                    transition: width 0.3s;
                }
                .center-3 {
                    width: 1000px;
                    margin: 0 auto;
                    display: grid;
                    row-gap: 15px;
                }
                .jigzi {
                    display: grid;
                    place-content: center;
                }
                h1 {
                    margin: 0;
                    font-size: 64px;
                    font-weight: 900;
                    color: #fff;
                    text-align: center;
                }
                h1 .creation {
                    color: var(--green-4);
                }
                h4 {
                    color: var(--dark-gray-6);
                    font-size: 32px;
                    font-weight: 300;
                    text-align: center;
                    margin: 0;
                }
                h4 .results-count {
                    font-weight: bold;
                }
                .help {
                    grid-column: 2;
                    grid-row: 1;
                }
            `,
        ];
    }

    @property({ reflect: true })
    mode: Mode = "home";

    @property({ type: Number })
    resultsCount: number = 0;

    renderSearchSection() {
        if (this.mode === 'home') {
            return html`
                <div class="center-1">
                    <div class="center-2">
                        <div class="center-3">
                            <img-ui
                                class="jigzi"
                                path="entry/home/search-section/jigzi.svg"
                            ></img-ui>
                            <h1>
                                ${STR_LEARNING}
                                <span class="creation">${STR_CREATION}</span>
                            </h1>
                            <h4>
                                ${STR_MAKE_LEARNING}
                                <span class="results-count"
                                    >${numberFormat(this.resultsCount)}</span
                                >
                                ${STR_JIGS}
                            </h4>
                            <slot name="search-bar"></slot>
                        </div>
                    </div>
                </div>
            `;
        } else {
            return html`
                <div class="center-1">
                    <div class="center-2">
                        <div class="center-3">
                            <slot name="search-bar"></slot>
                        </div>
                    </div>
                </div>
            `;
        }
    }

    renderHelp() {
        if (this.mode !== 'results') {
            return null;
        }

        // TODO: Enable once ready
        return html`
            <div class="help results-only">
                <slot name="help"></slot>
            </div>
        `;
    }

    render() {
        // 3 levels of center:
        //  1) take both grid columns
        //  2) full width in home and 0 width in result mode
        //  3) container of actual content
        return html`
            <div class="width-holder">
                <!--
                -->
                ${this.renderSearchSection()}
                ${this.renderHelp()}
            </div>
        `;
    }
}
