import { css, customElement, html, property } from "lit-element";
import { PopupBase } from "./popup-base";

const STR_TIMES_UP = "Time’s up!";

@customElement("jig-play-time-up-popup")
export class _ extends PopupBase {
    static get styles() {
        return [
            ...super.styles,
            css`
                :host {
                    grid-template-rows: 1fr auto;
                }
                .top-section {
                    grid-template-rows: 1fr auto;
                    overflow: hidden;
                }
                @media (min-width: 1024px) {
                    :host {
                        grid-template-rows: 1fr 164px;
                    }
                }
                img-ui {
                    height: 60px;
                    object-fit: contain;
                    justify-self: center;
                }
                @media (min-width: 1024px) {
                    img-ui {
                        height: 100%;
                        min-height: 0;
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
