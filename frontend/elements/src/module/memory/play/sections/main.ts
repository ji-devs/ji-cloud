import { LitElement, html, css, customElement, property, unsafeCSS } from "lit-element";
import {classMap} from "lit-html/directives/class-map";
import { mediaUi } from "@utils/path";

const GRID_LOOKUP:{[key: number]: number} = {
    8: 7,
    10: 7,
    12: 3,
    14: 7,
    16: 7,
    18: 7,
    20: 7,
    22: 7,
    24: 7,
    26: 7,

    //TODO above
    28: 7,
};
@customElement("play-main")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: flex;
                    height: 100%;
                    width: calc(1320rem * (1920/1719));
                    align-items: center;
                    justify-content: center;
                }
                section {
                    display: grid;
                    width: 100%;
                }

                .grid-3 {
                    grid-template-columns: repeat(3, auto); 
                    row-gap: calc(24rem * (1920/1719));
                    justify-content: space-around;
                }
                .grid-7 {
                    grid-template-columns: repeat(7, auto); 
                    row-gap: calc(24rem * (1920/1719));
                    justify-content: space-between;
                }

            `,
        ];
    }

    @property({type: Number})
    nCards:number = 0

    render() {
        const {nCards} = this;
        const gridNumber = GRID_LOOKUP[nCards];

        return html`
            <section class="grid-${gridNumber}">
                <slot></slot>
            </section>
        `
    }
}
