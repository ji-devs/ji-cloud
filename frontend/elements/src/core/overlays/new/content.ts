import { LitElement, html, css, customElement, property } from 'lit-element';
import {nothing} from "lit-html";
import { queryPierceShadow} from '@utils/dom';

export type MoveStrategy = "" | "dispatchClose" | "track";

//if it's a string then it will work as a querySelector
//unless it's "window" which will use the window (this is also the default for `container`) 
//the querySelector will work from the top of the document (and pierces through the shadowDom)
export type TrackerProp = TrackerSource | string | (() => TrackerSource);
export type TrackerSource = HTMLElement | DOMRect | Window;

/*

The main idea is we have 2 boxes, content and target:

tl--tm--tr
|       |
ml--mm--mr
|       |
bl--bm--mr


tl--tm--tr
|       |
ml--mm--mr
|       |
bl--bm--mr

setting the position will make them match up. So origin and content both being tl will pin to the top-left corner
as a helper, content can default to opposite across either axis

Lastly, a container target can be supplied with will be used to reposition things up to 3 times to try and better fit inside it
*/


//match it in overlay.rs on the rust side
export type Placement = 
    "tl" | "tm" | "tr" | "ml" | "mm" | "mr" | "bl" | "bm" | "mr";

export type ContentPlacement = Placement | "oppositeV" | "oppositeH" | "oppositeVH";

const MAX_RECURSE_DEPTH = 3;

export type ZLayer = "menu" | "tooltip" | "modal"

@customElement("overlay-content")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    position: absolute;
                    left: 0;
                    top: 0;
                    display: block;
                }

                :host([zLayer="menu"]) {
                    z-index: 1;
                }
                :host([zLayer="tooltip"]) {
                    z-index: 100;
                }
                :host([zLayer="modal"]) {
                    z-index: 200;
                }

            `
        ];
    }

    state:State | undefined;

    @property()
    container:TrackerProp | undefined = window;

    @property()
    target:TrackerProp | undefined;

    @property()
    strategy:MoveStrategy = "";

    @property({reflect: true})
    zLayer:ZLayer | undefined;

    @property()
    contentPlacement:ContentPlacement = "oppositeH";

    @property()
    targetPlacement:Placement = "tr";

    @property({type: Number})
    margin:number = 0;

    firstUpdated(_changed:any) {
        this.bindInstance();
    }

    updated(changed:any) {

        if(typeof changed.get("target") === "boolean") {
            this.bindInstance();
        }
    }

    bindInstance = () => {
        this.killInstance();

        if(this.target) {
            this.state = createState({
                target: new Tracker(this.target),
                content: new Tracker(this),
                container: new Tracker(this.container),
                targetPlacement: this.targetPlacement,
                contentPlacement: this.contentPlacement,
                margin: this.margin,
                strategy: this.strategy,
                dispatcher: this,
            });
        } else {
            console.info("no overlay-content target set");
        }
    }

    killInstance = () => {
        if(this.state != undefined) {
            this.state.destroy();
            this.state = undefined;
        }
    }

    disconnectedCallback() {
        this.killInstance();
    }


    render() {
        return html`
            <slot></slot>
        `;
    }
}

class Tracker {
    private source:TrackerSource | undefined;

    constructor(prop?:TrackerProp) {
        if(prop != null && prop !== "") {
            if(prop === "window") {
                this.source = window;
            }  else {
                const source:TrackerSource | null = typeof prop === "string" ? queryPierceShadow(document, prop)
                    : typeof prop === "function" ? prop()
                    : prop;

                if(source != null) {
                    this.source = source;
                }
            }
        }
    }

    public withElement<A>(f:(element:HTMLElement) => A): A | null {
        if(this.source instanceof HTMLElement) {
            return f(this.source);
        } else {
            return null;
        }
    }

    public observe(observer:ResizeObserver) {
        if(this.source instanceof HTMLElement) {
            observer.observe(this.source);
        } 
    }

    get valid():boolean {
        return this.source != undefined;
    }

    get domRect():DOMRect {
        if(this.source == undefined) {
            console.warn("invalid source! returning tiny DOMRect");
            return new DOMRect(0, 0, 0, 0);
        }

        if(this.source instanceof DOMRect) {
            return this.source;
        } else if(this.source instanceof Window) {
            return new DOMRect(0, 0, this.source.innerWidth, this.source.innerHeight);
        } else {
            return this.source.getBoundingClientRect();
        }
    }
}

interface State {
    destroy: () => any
}
interface StateOpts {
    target: Tracker, 
    content: Tracker, 
    container: Tracker, 
    contentPlacement: ContentPlacement,
    targetPlacement: Placement,
    margin: number,
    strategy: MoveStrategy,
    dispatcher: EventTarget,
}
function createState(opts:StateOpts):State {
    let lastTargetRect:DOMRect | undefined;

    const {target, margin, container, content, strategy} = opts;

    const _recalc = (contentPlacement: ContentPlacement, targetPlacement:Placement, recurseDepth: number) => {
        if(recurseDepth === 0) {
            return;
        }

        type V = "t" | "m" | "b";
        type H = "l" | "m" | "r";
        let contentV:V = contentPlacement[0] as V;
        let contentH:H = contentPlacement[1] as H;
        const targetV:V = targetPlacement[0] as V;
        const targetH:H = targetPlacement[1] as H;

        //should this keep flipping after first call? o_O
        if(contentPlacement === "oppositeV" || contentPlacement === "oppositeVH") {
            contentV = targetV === "t" ? "b" 
                : targetV === "b" ? "t"
                : "m";
            if(contentPlacement !== "oppositeVH") {
                contentH = targetH;
            }
        }
        if(contentPlacement === "oppositeH" || contentPlacement === "oppositeVH") {
            contentH = targetH === "l" ? "r" 
                : targetH === "r" ? "l"
                : "m";

            if(contentPlacement !== "oppositeVH") {
                contentV = targetV;
            }
        }

        const targetRect = target.domRect; 
        const contentRect = content.domRect; 


        /// Horizontal axis
        /// Margin only pushes from opposit sides
        let x:number = 0;
        if(targetH === "l") {
            x = targetRect.x;
        } else if(targetH === "m") {
            x = targetRect.x + targetRect.width/2;
        } else if(targetH === "r") {
            x = targetRect.x + targetRect.width;
        }
        if(contentH === "l") {
            if(targetH === "r") {
                x += margin;
            }
        } else if(contentH === "m") {
            x -= contentRect.width/2;
        } else if(contentH === "r") {
            x -= contentRect.width;
            if(targetH === "l") {
                x -= margin;
            }
        }

        let y:number = 0;
        if(targetV === "t") {
            y = targetRect.y;
        } else if(targetV === "m") {
            y = targetRect.y + targetRect.height/2;
        } else if(targetV === "b") {
            y = targetRect.y + targetRect.height;
        }
        if(contentV === "t") {
            if(targetV === "b") {
                y += margin;
            }
        } else if(contentV === "m") {
            y -= contentRect.height/2;
        } else if(contentV === "b") {
            y -= contentRect.height;
            if(targetV === "t") {
                y -= margin;
            }
        }

        if(container.valid) {
            const containerRect = container.domRect; 

            const lastResort:boolean = recurseDepth <= 1;

            let newContentH:H | undefined; 
            let newContentV:V | undefined; 
            let newTargetH:H | undefined; 
            let newTargetV:V | undefined; 

            if((x + contentRect.width) > containerRect.right) {
                if(lastResort) {
                    x = containerRect.right - (contentRect.width + margin);
                } else {
                    newTargetH = "l";
                    newContentH = "r";
                }
            }
            if((y + contentRect.height) > containerRect.bottom) {
                if(lastResort) {
                    y = containerRect.bottom - (contentRect.height + margin);
                } else {
                    newTargetV = "t";
                    newContentV = "b";
                }
            }
            if(x < containerRect.left) {
                if(lastResort) {
                    x = containerRect.x + margin;
                } else {
                    newTargetH = "r";
                    newContentH = "l";
                }
            }
            if(y < containerRect.top) {
                if(lastResort) {
                    y = containerRect.top + margin;
                } else {
                    newTargetV = "b";
                    newContentV = "t";
                }
            }


            if(newContentH || newContentV || newTargetV || newTargetV) {
                let replaceContent = `${newContentV || contentV}${newContentH || contentH}`;
                let replaceTarget = `${newTargetV || targetV}${newTargetH || targetH}`;

                _recalc(replaceContent as Placement, replaceTarget as Placement, recurseDepth-1);
                return;
            }
        }

        content.withElement(element => {
            let style:CSSStyleDeclaration = element.style;

            style.setProperty('top', `${y}px`);
            style.setProperty('left', `${x}px`);

        });
    }

    const recalc = () => _recalc(opts.contentPlacement, opts.targetPlacement, MAX_RECURSE_DEPTH);
    // @ts-ignore
    const observer = new ResizeObserver(recalc);

    target.observe(observer);
    content.observe(observer);
    container.observe(observer);

    //very inefficient, but ResizeObserver doesn't take this into account
    let rafId:number | undefined;
    if(strategy !== "") {
        const checkPosition = () => {

            const targetRect = target.domRect;

            if(lastTargetRect !== undefined) {
                if(targetRect.x !== lastTargetRect.x || targetRect.y !== lastTargetRect.y) {
                    if(strategy === "track") {
                        recalc();
                    } else if(strategy === "dispatchClose") {
                        opts.dispatcher.dispatchEvent(new Event("close"));
                    }
                }
            }
            lastTargetRect = targetRect;
            rafId = requestAnimationFrame(checkPosition);
        }

        rafId = requestAnimationFrame(checkPosition);
    }


    window.addEventListener("resize", recalc);

    const destroy = () => {
        if(rafId != undefined) {
            cancelAnimationFrame(rafId)
        }
        observer.disconnect();
        window.removeEventListener("resize", recalc);
    }

    recalc();

    return {
        destroy
    }
}
