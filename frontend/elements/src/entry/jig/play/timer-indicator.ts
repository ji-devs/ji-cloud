import { css, customElement, html } from "lit-element";
import { IndicatorBase } from "./indicator-base";

const ANIMATION_DURATION = 500;

@customElement("jig-play-timer-indicator")
export class _ extends IndicatorBase {
    static get styles() {
        return [...super.styles, css`
            :host(.buzz) {
                animation-name: buzz;
                animation-duration: ${ANIMATION_DURATION}ms;
                transform-origin: 50% 100%;
            }
            @keyframes buzz {
                0% { transform: rotate(0deg); }
                10% { transform: rotate(3deg); }
                20% { transform: rotate(-3deg); }
                30% { transform: rotate(3deg); }
                40% { transform: rotate(-3deg); }
                50% { transform: rotate(3deg); }
                60% { transform: rotate(-2deg); }
                70% { transform: rotate(2deg); }
                80% { transform: rotate(-1deg); }
                90% { transform: rotate(1deg); }
                100% { transform: rotate(0deg); }
            }

            :host(:not(.buzz)) .img-gold,
            :host(.buzz) .img-blue {
                display: none;
            }
        `]
    }

    buzz() {
        this.classList.add("buzz");
        setTimeout(() => {
            this.classList.remove("buzz");
        }, ANIMATION_DURATION);
    }

    render() {
        return this.renderIndicator(() => {
            return html`
                <img-ui path="entry/jig/play/timer.svg"></img-ui>
            `
        });
    }
}
