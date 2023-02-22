import { css, customElement, html, property } from "lit-element";
import { nothing } from "lit-html";
import { PopupBase } from "./popup-base";

const STR_SCORE = "SCORE";

@customElement("jig-play-done-popup")
export class _ extends PopupBase {
    static get styles() {
        return [
            ...super.styles,
            css`
                img-ui {
                    height: 60px;
                }
                @media (min-width: 1024px) {
                    img-ui {
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
            `,
        ];
    }

    @property({ type: Number })
    score?: number;

    render() {
        return html`
            ${this.renderBase(() => {
                return html`
                    <img-ui path="entry/jig/play/jig-finish.gif"></img-ui>
                    ${this.score !== undefined
                        ? html`
                            <div class="score-section">
                                <h3>${STR_SCORE}</h3>
                                <h2>${this.score}</h2>
                            </div>
                        ` : nothing}
                `;
            })}
        `;
    }
}
