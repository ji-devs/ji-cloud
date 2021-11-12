import {
    LitElement,
    html,
    css,
    customElement,
    property,
    unsafeCSS,
} from "lit-element";
import { classMap } from "lit-html/directives/class-map";

@customElement("play-main")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: flex;
                    height: 100%;
                    width: 1474rem;
                    align-items: center;
                    justify-content: center;
                }

                section {
                    display: grid;
                    width: 100%;
                    justify-content: center;
                    gap: 26rem;
                }

                .cols-3 {
                    grid-template-columns: repeat(3, auto);
                }
                .cols-4 {
                    grid-template-columns: repeat(4, auto);
                }
                .cols-5 {
                    grid-template-columns: repeat(5, auto);
                }
                .cols-6 {
                    grid-template-columns: repeat(6, auto);
                }
                .cols-7 {
                    grid-template-columns: repeat(7, auto);
                }

                .last-10 > ::slotted(:nth-child(9)) {
                    grid-column: 2;
                }

                .last-14 > ::slotted(:nth-child(13)) {
                    grid-column: 2;
                }

                .last-18 > ::slotted(:nth-child(16)) {
                    grid-column: 2;
                }
                .last-22 > ::slotted(:nth-child(19)) {
                    grid-column: 2;
                }
                .last-26 > ::slotted(:nth-child(22)) {
                    grid-column: 2;
                }
            `,
        ];
    }

    @property({ type: Number })
    nCards: number = 0;

    render() {
        const { nCards } = this;

        const classes: any = {};

        if (nCards == 6) {
            classes["cols-3"] = true;
        } else if (nCards < 17) {
            classes["cols-4"] = true;
        } else if (nCards < 21) {
            classes["cols-5"] = true;
        } else if (nCards < 25) {
            classes["cols-6"] = true;
        } else {
            classes["cols-7"] = true;
        }

        switch (nCards) {
            case 10:
                classes["last-10"] = true;
                break;
            case 14:
                classes["last-14"] = true;
                break;
            case 18:
                classes["last-18"] = true;
                break;
            case 22:
                classes["last-22"] = true;
                break;
            case 26:
                classes["last-26"] = true;
                break;
            default:
                break;
        }

        return html`
            <section class=${classMap(classes)}>
                <slot></slot>
            </section>
        `;
    }
}
