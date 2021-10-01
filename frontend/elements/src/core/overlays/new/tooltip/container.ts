import { LitElement, svg, html, css, customElement, property } from "lit-element";
import { nothing } from "lit-html";
import { styleMap } from "lit-html/directives/style-map";
import { Anchor, ContentAnchor, getAnchors } from "@elements/core/overlays/new/content";

const TRIANGLE_WIDTH = 18;
const TRIANGLE_HEIGHT = 10;
const OUTLINE_SIZE = 3;

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
                :host([arrowAnchor="tr"])
                {
                    flex-direction: row;
                    align-items: flex-start;
                    justify-content: center;
                }

                :host([color="green"]) > .main {
                    border: solid 2px #4bb972;
                    background-color: var(--main-green);
                }
                :host([color="red"]) > .main {
                    background-color: var(--light-red-1);
                }

                :host([color="beige"]) > .main {
                    border: solid 2px var(--light-orange-2);
                    background-color: var(--light-orange-1);
                }

                /* beige */
                :host([color="beige"]) .tri {
                    fill: var(--light-orange-1);
                }
                :host([color="beige"]) .tri path {
                    stroke: var(--light-orange-1);
                }
                :host([color="beige"]) .tri-repaint path {
                    stroke: var(--light-orange-2);
                }

                /* arrow offsets 
                :host([arrowAnchor="tr"]) > .main {
                    margin-right: ${TRIANGLE_HEIGHT}px;
                }
                :host([arrowAnchor="tl"]) > .main {
                    margin-left: ${TRIANGLE_HEIGHT}px;
                }
                :host([arrowAnchor="tm"]) > .main {
                    margin-top: ${TRIANGLE_HEIGHT}px;
                }
                */

                /* main */
                .main {
                    grid-area: mm;
                    border-radius: 25rem;
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
        const {targetH, targetV, contentH, contentV} = getAnchors(this.contentAnchor, this.targetAnchor);
        this.arrowAnchor = `${contentV}${contentH}` as Anchor;
    }
    
    @property({ reflect: true })
    color: string = "beige";

    @property()
    contentAnchor: ContentAnchor = "oppositeH";

    @property()
    targetAnchor: Anchor = "tr";

    @property({type: Number})
    arrowNudge: number = 0; 

    //computed
    @property({reflect: true})
    arrowAnchor: Anchor | undefined;
    
    render() {
        const { arrowAnchor, arrowNudge, contentAnchor, targetAnchor } = this;

        if(!arrowAnchor) {
            return nothing;
        }
        
        return html`
            ${renderArrow(arrowAnchor, arrowNudge, true)}
            <div class="main">
                <slot></slot>
            </div>
            ${renderArrow(arrowAnchor, arrowNudge, false)}
        `;
    }
}

function renderArrow(arrowAnchor:Anchor, userArrowNudge:number, isFirst: boolean) {

    const FIRST_MAP:any = {
        tl: true,
        tm: true,
        tr: false,
    }
    if((isFirst && !FIRST_MAP[arrowAnchor]) || (!isFirst && FIRST_MAP[arrowAnchor])) {
        return nothing;
    }

	const boxWidth = TRIANGLE_WIDTH + (OUTLINE_SIZE * 2); 
	const boxHeight = TRIANGLE_HEIGHT + (OUTLINE_SIZE * 2);

	const left = OUTLINE_SIZE;
	const right = OUTLINE_SIZE + TRIANGLE_WIDTH;
	const middle = OUTLINE_SIZE + (TRIANGLE_WIDTH/ 2); 
	const bottom = OUTLINE_SIZE + TRIANGLE_HEIGHT;
	const top = OUTLINE_SIZE;

    const DEFAULT_ARROW_NUDGE:any = {
        tl: 10, 
        tm: 0,
        tr: 10, 
    }

    const arrowNudge = DEFAULT_ARROW_NUDGE[arrowAnchor] + userArrowNudge;

    const ROT_MAP:any = {
        tl: -90,
        tm: 0,
        tr: 90,
    }

    const TX_MAP:any = {
        tl: (boxHeight/2)+OUTLINE_SIZE,
        tm: arrowNudge,
        tr: -((boxHeight)+OUTLINE_SIZE),
    }
    const TY_MAP:any = {
        tl: arrowNudge,
        tm: boxHeight,
        tr: arrowNudge,
    }
    const CW_MAP:any = {
        tl: boxHeight,
        tm: boxWidth,
        tr: boxHeight,
    }

    const CH_MAP:any = {
        tl: boxWidth,
        tm: boxHeight,
        tr: boxWidth,
    }
    const IW_MAP:any = {
        tl: boxWidth,
        tm: boxWidth,
        tr: boxWidth,
    }

    const IH_MAP:any = {
        tl: boxHeight,
        tm: boxHeight,
        tr: boxHeight,
    }

    //const style = `transform: rotate(${ROT_MAP[arrowAnchor]}deg) translateX(${TX_MAP[arrowAnchor]}px) translateY(${TY_MAP[arrowAnchor]}px)`;

    //const containerStyle = `grid-area: ${arrowAnchor};`
    //const containerStyle = ``;
    const containerStyle= `position: relative; transform: translateX(${TX_MAP[arrowAnchor]}px) translateY(${TY_MAP[arrowAnchor]}px); width: ${CW_MAP[arrowAnchor]}px; height: ${CH_MAP[arrowAnchor]}px;`;
    const style = `position: absolute; transform: rotate(${ROT_MAP[arrowAnchor]}deg); width: ${IW_MAP[arrowAnchor]}px; height: ${IH_MAP[arrowAnchor]}px;`;
    //const style = `width: ${CW_MAP[arrowAnchor]}px; height: ${CH_MAP[arrowAnchor]}px;`;
    console.log(arrowAnchor, style)
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
                    <path d="M ${left},${bottom - (OUTLINE_SIZE/2)} ${middle},${top} z"/>
            </svg>
            <svg xmlns="http://www.w3.org/2000/svg" version="1.1" class="tri-repaint" width='${boxWidth}' height='${boxHeight}'>
                    <path d="M 0,${bottom - (OUTLINE_SIZE/2)} ${left},${bottom - (OUTLINE_SIZE/2)} z"/>
            </svg>
            <svg xmlns="http://www.w3.org/2000/svg" version="1.1" class="tri-repaint" width='${boxWidth}' height='${boxHeight}'>
                    <path d="M ${right},${bottom - (OUTLINE_SIZE/2)} ${middle},${top} z"/>
            </svg>
            <svg xmlns="http://www.w3.org/2000/svg" version="1.1" class="tri-repaint" width='${boxWidth}' height='${boxHeight}'>
                    <path d="M ${boxWidth},${bottom - (OUTLINE_SIZE/2)} ${right},${bottom - (OUTLINE_SIZE/2)} z"/>
            </svg>
		  `}
        </div>
        </div>`;
}