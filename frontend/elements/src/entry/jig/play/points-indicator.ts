import { css, customElement, html, PropertyValues } from "lit-element";
import { IndicatorBase } from "./indicator-base";

const ANIMATION_DURATION = 100;
const ANIMATION_COUNT = 4;

@customElement("jig-play-points-indicator")
export class _ extends IndicatorBase {
    static get styles() {
        return [...super.styles, css`
            :host(.pop) {
                animation-name: pop;
                animation-duration: ${ANIMATION_DURATION}ms;
                animation-direction: alternate;
                animation-iteration-count: ${ANIMATION_COUNT};
            }
            @keyframes pop {
                from {
                    transform: scale(1);
                }
                to {
                    transform: scale(1.1);
                }
            }

            :host(:not(.pop)) .img-gold,
            :host(.pop) .img-blue {
                display: none;
            }
        `]
    }

    updated(changedProperties: PropertyValues) {
        if(changedProperties.has("value")) {
            this.valueChanged()
        }
    }

    private valueChanged() {
        if((this.value as any) === 0) return;

        this.classList.add("pop");
        setTimeout(() => {
            this.classList.remove("pop");
        }, ANIMATION_DURATION * ANIMATION_COUNT);
    }

    render() {
        return this.renderIndicator(() => {
            return html`
                <img-ui class="img-gold" path="entry/jig/play/trophy-gold.svg"></img-ui>
                <img-ui class="img-blue" path="entry/jig/play/trophy-blue.svg"></img-ui>
            `
        });
    }
}
