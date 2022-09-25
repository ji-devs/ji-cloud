import { css, customElement, html, property } from "lit-element";
import { PopupBase } from "./popup-base";

const STR_TIMES_UP = "Time’s up!";

@customElement("jig-play-time-up-popup")
export class _ extends PopupBase {
    static get styles() {
        return [
            ...super.styles,
            css`
                h2 {
                    margin: 0;
                    font-size: 56px;
                    font-weight: bold;
                    color: var(--light-orange-6);
                }
            `,
        ];
    }

    render() {
        return html`
            ${this.renderBase(() => {
                return html`
                    <img-ui path="entry/jig/play/time-up.png"></img-ui>
                    <h2>${STR_TIMES_UP}</h2>
                `;
            })}
        `;
    }
}
