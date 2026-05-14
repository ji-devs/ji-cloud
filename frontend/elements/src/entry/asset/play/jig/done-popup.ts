import { css, customElement, html, property } from "lit-element";
import { nothing } from "lit-html";
import { PopupBase } from "./popup-base";

const STR_SCORE = "SCORE";
const GOLD_CUP_PATH = "entry/jig/play/jig-finish.gif";
const SILVER_CUP_PATH = "entry/jig/play/jig-finish-silver.gif";
const BRONZE_CUP_PATH = "entry/jig/play/jig-finish-bronze.gif";

@customElement("jig-play-done-popup")
export class _ extends PopupBase {
    static get styles() {
        return [
            ...super.styles,
            css`
                .finish-image {
                    height: 60px;
                }
                .finish-image img-ui {
                    height: 100%;
                }
                @media (min-width: 1024px) {
                    .finish-image {
                        height: 350px;
                    }
                }
                h3 {
                    margin: 0;
                    font-size: 16px;
                    font-weight: bold;
                    color: var(--light-orange-6);
                }
                @media (min-width: 1024px) {
                    h3 {
                        font-size: 22px;
                    }
                }
                h2 {
                    margin: 0;
                    font-size: 30px;
                    font-weight: 600;
                    font-style: italic;
                    color: #ffffff;
                }
                @media (min-width: 1024px) {
                    h2 {
                        font-size: 64px;
                    }
                }
                .percentage {
                    font-size: 22px;
                    font-weight: 600;
                    font-style: italic;
                    color: var(--light-orange-6);
                }
                @media (min-width: 1024px) {
                    .percentage {
                        font-size: 40px;
                    }
                }
            `,
        ];
    }

    @property({ type: Number })
    score?: number;

    @property({ type: Number })
    percentage?: number;

    private finishImagePath(): string | null {
        if (this.percentage === undefined) {
            return GOLD_CUP_PATH;
        }
        if (this.percentage === 0) {
            return null;
        }
        if (this.percentage <= 33) {
            return BRONZE_CUP_PATH;
        }
        if (this.percentage <= 66) {
            return SILVER_CUP_PATH;
        }
        return GOLD_CUP_PATH;
    }

    render() {
        const imagePath = this.finishImagePath();

        return html`
            ${this.renderBase(() => {
                return html`
                    <div class="finish-image">
                        ${imagePath === null
                            ? nothing
                            : html`<img-ui path="${imagePath}"></img-ui>`}
                    </div>
                    ${this.score !== undefined
                        ? html`
                            <div class="score-section">
                                <h3>${STR_SCORE}</h3>
                                <h2>
                                    ${this.score}
                                    ${this.percentage !== undefined
                                        ? html` <span class="percentage">(${this.percentage}%)</span>`
                                        : nothing}
                                </h2>
                            </div>
                        ` : nothing}
                `;
            })}
        `;
    }
}
