import { css, customElement, html, property } from "lit-element";
import { PopupBase } from "./popup-base";

const STR_TIMES_UP = "Time’s up!";

@customElement("jig-play-time-up-popup")
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
                h2 {
                    margin: 0;
                    font-size: 24px;
                    font-weight: bold;
                    color: var(--light-orange-6);
                }
                @media (min-width: 1024px) {
                    h2 {
                        font-size: 56px;
                    }
                }
            `,
        ];
    }

    render() {
        return html`
            ${this.renderBase(() => {
                return html`
                    <img-ui path="entry/jig/play/time-up.svg"></img-ui>
                    <h2>${STR_TIMES_UP}</h2>
                `;
            })}
        `;
    }
}
