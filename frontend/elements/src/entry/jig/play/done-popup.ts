import { css, customElement, html, property } from "lit-element";
import { nothing } from "lit-html";
import { PopupBase } from "./popup-base";

const STR_SCORE = "SCORE";

@customElement("jig-play-done-popup")
export class _ extends PopupBase {
    static get styles() {
        return [...super.styles, css`
            .images {
                display: grid;
            }
            .images .trophy {
                width: 100px;
                place-self: center;
            }
            h3 {
                margin: 0;
                font-size: 22px;
                font-weight: bold;
                color: var(--light-orange-6);
            }
            h2 {
                margin: 0;
                font-size: 64px;
                font-weight: 600;
                font-style: italic;
                color: #ffffff;
            }
        `]
    }

    @property({ type: Number })
    score?: number;

    render() {
        return html`
            ${this.renderBase(() =>{
                return html`
                    <div class="images">
                        <img-ui path="entry/jig/play/party-poppers.svg"></img-ui>
                        <img-ui class="trophy" path="entry/jig/play/trophy-gold.svg"></img-ui>
                    </div>
                    ${ this.score !== undefined ? html`
                        <div class="score-section">
                            <h3>${STR_SCORE}</h3>
                            <h2>${this.score}</h2>
                        </div>
                    ` : nothing }
                `;
            })}
        `
    }
}
