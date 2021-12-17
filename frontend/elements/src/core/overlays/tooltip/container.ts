// This expands on overlay-content to allow for the arrows
// internally it adds some additional anchors to allow positioning
// the arrows on the top or vertical corners, above the content (like tm/bm)
//
// the "nudge" is generally

import {
    LitElement,
    svg,
    html,
    css,
    customElement,
    property,
} from "lit-element";
import { nothing } from "lit-html";
import {
    Anchor,
    ContentAnchor,
    getAnchors,
} from "@elements/core/overlays/content";

export type Color = "blue" | "red" | "green";

const TRIANGLE_WIDTH = 18;
const TRIANGLE_HEIGHT = 10;
const OUTLINE_SIZE = 3;
const BASE_ARROW_NUDGE = 24;

type ArrowAnchor = Anchor | "ttr" | "ttl" | "bbr" | "bbl";

@customElement("tooltip-container")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: flex;
                    flex-direction: column;
                    align-items: center;
                    justify-content: center;
                }

                :host([arrowAnchor="tl"]),
                :host([arrowAnchor="tr"]),
                :host([arrowAnchor="bl"]),
                :host([arrowAnchor="br"]),
                :host([arrowAnchor="ml"]),
                :host([arrowAnchor="mr"]) {
                    flex-direction: row;
                }

                :host([arrowAnchor="tl"]),
                :host([arrowAnchor="tr"]) {
                    align-items: flex-start;
                }
                :host([arrowAnchor="ttl"]),
                :host([arrowAnchor="bbl"]) {
                    align-items: flex-start;
                }
                :host([arrowAnchor="bl"]),
                :host([arrowAnchor="br"]),
                :host([arrowAnchor="ttr"]),
                :host([arrowAnchor="bbr"]) {
                    align-items: flex-end;
                }

                :host([color="blue"]) {
                    --tooltip-bg-color: var(--light-orange-1);
                    --tooltip-border-color: var(--light-orange-2);

                    --tooltip-bg-color: var(--dark-blue-5);
                    --tooltip-border-color: var(--light-blue-3);
                }
                :host([color="green"]) {
                    --tooltip-bg-color: var(--main-green);
                    --tooltip-border-color: #4bb972;
                }
                :host([color="red"]) {
                    --tooltip-bg-color: var(--light-red-1);
                    --tooltip-border-color: var(--light-red-1);
                }

                .main {
                    background-color: var(--tooltip-bg-color);
                    border: solid 3px var(--tooltip-border-color);
                }
                .tri {
                    fill: var(--tooltip-bg-color);
                }
                .tri path {
                    stroke: var(--tooltip-bg-color);
                }
                .tri-repaint path {
                    stroke: var(--tooltip-border-color);
                }

                /* arrow offsets 
                :host([arrowAnchor="tr"]) > .main {
                    margin-right: ${TRIANGLE_HEIGHT}px;
                }
                :host([arrowAnchor="tl"]) > .main {
                    margin-left: ${TRIANGLE_HEIGHT}px;
                }
                :host([arrowAnchor="tm"]) > .main,
                :host([arrowAnchor="ttr"]) > .main,
                :host([arrowAnchor="ttl"]) > .main,
                {
                    margin-top: ${TRIANGLE_HEIGHT}px;
                }
                */

                /* main */
                .main {
                    border-radius: 25px;
                    padding: 12px 24px;
                }

                /* triangle */
                .tri,
                .tri-repaint {
                    position: absolute;
                    top: -${TRIANGLE_HEIGHT}px;
                }

                .tri-repaint {
                    fill-opacity: 0;
                }

                .tri path {
                    stroke-width: ${OUTLINE_SIZE};
                    stroke-linejoin: round;
                    stroke-linecap: round;
                }

                .tri-repaint path {
                    stroke-width: ${OUTLINE_SIZE};
                    stroke-linejoin: round;
                    stroke-linecap: round;
                }
            `,
        ];
    }

    firstUpdated() {
        this.updateAnchor();
    }

    updated() {
        this.updateAnchor();
    }

    updateAnchor() {
        let { targetH, targetV, contentH, contentV } = getAnchors(
            this.contentAnchor,
            this.targetAnchor
        );

        if (contentV == "t" && contentH != "m" && targetV == "b") {
            contentV += "t";
        } else if (contentV == "b" && contentH != "m" && targetV == "t") {
            contentV += "b";
        }
        this.arrowAnchor = `${contentV}${contentH}` as Anchor;
    }

    @property({ reflect: true })
    color: Color = "blue";

    @property()
    contentAnchor: ContentAnchor = "oppositeH";

    @property()
    targetAnchor: Anchor = "tr";

    @property({ type: Number })
    arrowNudge: number = 0;

    //computed - do not set manually!
    @property({ reflect: true })
    arrowAnchor: ArrowAnchor | undefined;

    render() {
        const { arrowAnchor, arrowNudge, contentAnchor, targetAnchor } = this;

        if (!arrowAnchor) {
            return nothing;
        }

        const showArrow = arrowAnchor != "mm";

        return html`
            ${showArrow ? renderArrow(arrowAnchor, arrowNudge, true) : nothing}
            <div class="main">
                <slot></slot>
            </div>
            ${showArrow ? renderArrow(arrowAnchor, arrowNudge, false) : nothing}
        `;
    }
}

function renderArrow(
    arrowAnchor: ArrowAnchor,
    userArrowNudge: number,
    isFirst: boolean
) {
    // whether to skip the first render
    // first vs. second has to do with the flexbox side
    const FIRST_MAP: any = {
        ml: true,
        mr: false,
        tl: true,
        tm: true,
        ttl: true,
        ttr: true,
        tr: false,
        bl: true,
        bm: false,
        bbl: false,
        bbr: false,
        br: false,
    };
    if (
        (isFirst && !FIRST_MAP[arrowAnchor]) ||
        (!isFirst && FIRST_MAP[arrowAnchor])
    ) {
        return nothing;
    }

    // the exact rules here are more about adjusting so it looks right
    // rather than it making logical sense :p
    // feel free to change, but remember to test!

    const boxWidth = TRIANGLE_WIDTH + OUTLINE_SIZE * 2;
    const boxHeight = TRIANGLE_HEIGHT + OUTLINE_SIZE * 2;

    const left = OUTLINE_SIZE;
    const right = OUTLINE_SIZE + TRIANGLE_WIDTH;
    const middle = OUTLINE_SIZE + TRIANGLE_WIDTH / 2;
    const bottom = OUTLINE_SIZE + TRIANGLE_HEIGHT;
    const top = OUTLINE_SIZE;

    const DEFAULT_ARROW_NUDGE: any = {
        ml: 0,
        mr: 0,
        tl: BASE_ARROW_NUDGE,
        tm: 0,
        ttl: BASE_ARROW_NUDGE,
        ttr: BASE_ARROW_NUDGE,
        tr: BASE_ARROW_NUDGE,
        bl: BASE_ARROW_NUDGE,
        bm: 0,
        bbl: BASE_ARROW_NUDGE,
        bbr: BASE_ARROW_NUDGE,
        br: BASE_ARROW_NUDGE,
    };
    const FLIP_ARROW_NUDGE: any = {
        ml: false,
        mr: false,
        tl: false,
        tm: false,
        ttl: false,
        ttr: true,
        tr: false,
        bl: true,
        bm: false,
        bbl: false,
        bbr: true,
        br: true,
    };

    let arrowNudge = DEFAULT_ARROW_NUDGE[arrowAnchor] + userArrowNudge;
    if (FLIP_ARROW_NUDGE[arrowAnchor]) {
        arrowNudge *= -1;
    }

    const ROT_MAP: any = {
        ml: -90,
        mr: 90,
        tl: -90,
        tm: 0,
        ttr: 0,
        ttl: 0,
        tr: 90,
        bl: -90,
        bm: 180,
        bbr: 180,
        bbl: 180,
        br: 90,
    };

    const TX_MAP: any = {
        ml: boxHeight / 2 + OUTLINE_SIZE,
        mr: -(boxHeight + OUTLINE_SIZE),
        tl: boxHeight / 2 + OUTLINE_SIZE,
        //tl: OUTLINE_SIZE * 1.5,
        tm: arrowNudge,
        ttl: arrowNudge,
        ttr: arrowNudge,
        tr: -(boxHeight + OUTLINE_SIZE),
        bl: boxHeight / 2 + OUTLINE_SIZE,
        bm: arrowNudge,
        bbl: arrowNudge,
        bbr: arrowNudge,
        br: -(boxHeight + OUTLINE_SIZE),
    };
    const TY_MAP: any = {
        ml: arrowNudge,
        mr: arrowNudge,
        tl: arrowNudge,
        tm: boxHeight,
        ttl: boxHeight,
        ttr: boxHeight,
        tr: arrowNudge,
        bl: arrowNudge,
        bm: -(boxHeight / 2 + OUTLINE_SIZE * 2),
        bbl: -(boxHeight / 2 + OUTLINE_SIZE * 2),
        bbr: -(boxHeight / 2 + OUTLINE_SIZE * 2),
        br: arrowNudge,
    };
    const CW_MAP: any = {
        ml: boxHeight,
        mr: boxHeight,
        tl: boxHeight,
        tm: boxWidth,
        ttl: boxWidth,
        ttr: boxWidth,
        tr: boxHeight,
        bl: boxHeight,
        bm: boxWidth,
        bbl: boxWidth,
        bbr: boxWidth,
        br: boxHeight,
    };

    const CH_MAP: any = {
        ml: boxWidth,
        mr: boxWidth,
        tl: boxWidth,
        tm: boxHeight,
        ttl: boxHeight,
        ttr: boxHeight,
        tr: boxWidth,
        bl: boxWidth,
        bm: boxHeight,
        bbl: boxHeight,
        bbr: boxHeight,
        br: boxWidth,
    };

    //const style = `transform: rotate(${ROT_MAP[arrowAnchor]}deg) translateX(${TX_MAP[arrowAnchor]}px) translateY(${TY_MAP[arrowAnchor]}px)`;

    //const containerStyle = `grid-area: ${arrowAnchor};`
    //const containerStyle = ``;
    const containerStyle = `position: relative; transform: translateX(${TX_MAP[arrowAnchor]}px) translateY(${TY_MAP[arrowAnchor]}px); width: ${CW_MAP[arrowAnchor]}px; height: ${CH_MAP[arrowAnchor]}px;`;
    const style = `position: absolute; transform: rotate(${ROT_MAP[arrowAnchor]}deg); width: ${boxWidth}px; height: ${boxHeight}px;`;
    //const style = `width: ${CW_MAP[arrowAnchor]}px; height: ${CH_MAP[arrowAnchor]}px;`;
    console.log(arrowAnchor, style);
    //First draw the triangle with no outline
    //then draw the outlines on right and left sides
    //then fill in the small gap left by the hole due to round edges
    //<svg xmlns="http://www.w3.org/2000/svg" transform="rotate(${ROT_MAP[arrowAnchor]})" version="1.1" class="tri" width='${boxWidth}' height='${boxHeight}'>
    return html`<div style="${containerStyle}">
        <div style="${style}">
            ${svg`
            <svg xmlns="http://www.w3.org/2000/svg" version="1.1" class="tri" width='${boxWidth}' height='${boxHeight}'>
                    <path d="M ${left},${bottom} ${middle},${top} ${right},${bottom} z"/>
            </svg>
            <svg xmlns="http://www.w3.org/2000/svg" version="1.1" class="tri-repaint" width='${boxWidth}' height='${boxHeight}'>
                    <path d="M ${left},${
                bottom - OUTLINE_SIZE / 2
            } ${middle},${top} z"/>
            </svg>
            <svg xmlns="http://www.w3.org/2000/svg" version="1.1" class="tri-repaint" width='${boxWidth}' height='${boxHeight}'>
                    <path d="M 0,${bottom - OUTLINE_SIZE / 2} ${left},${
                bottom - OUTLINE_SIZE / 2
            } z"/>
            </svg>
            <svg xmlns="http://www.w3.org/2000/svg" version="1.1" class="tri-repaint" width='${boxWidth}' height='${boxHeight}'>
                    <path d="M ${right},${
                bottom - OUTLINE_SIZE / 2
            } ${middle},${top} z"/>
            </svg>
            <svg xmlns="http://www.w3.org/2000/svg" version="1.1" class="tri-repaint" width='${boxWidth}' height='${boxHeight}'>
                    <path d="M ${boxWidth},${
                bottom - OUTLINE_SIZE / 2
            } ${right},${bottom - OUTLINE_SIZE / 2} z"/>
            </svg>
		  `}
        </div>
    </div>`;
}
