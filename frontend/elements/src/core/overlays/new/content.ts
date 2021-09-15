import { LitElement, html, css, customElement, property } from 'lit-element';
import {nothing} from "lit-html";
import { queryPierceShadow} from '@utils/dom';

export type MoveStrategy = "" | "dispatchClose" | "track";

//if it's a string then it will work as a querySelector
//unless it's "window" which will use the window (this is also the default for `container`) 
//the querySelector will work from the top of the document (and pierces through the shadowDom)
export type TrackerProp = TrackerSource | string | (() => TrackerSource);
export type TrackerSource = HTMLElement | DOMRect | Window;

//match it in overlay.rs on the rust side
export type Placement = 
    "top"
    | "top-start"
    | "top-end"
    | "bottom"
    | "bottom-start"
    | "bottom-end"
    | "right"
    | "right-start"
    | "right-end"
    | "left"
    | "left-start"
    | "left-end";



@customElement("overlay-content")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    position: absolute;
                    left: 0;
                    top: 0;
                    display: inline-block;
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


    @property()
    placement:Placement = "left";

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
                placement: this.placement,
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
    placement: Placement,
    margin: number,
    strategy: MoveStrategy,
    dispatcher: EventTarget,
}
function createState(opts:StateOpts):State {
    let lastTargetRect:DOMRect | undefined;

    const {target, margin, container, content, strategy} = opts;

    const _recalc = (placement: Placement, recurseDepth: number) => {
        if(recurseDepth === 0) {
            return;
        }

        const splitIndex = placement.indexOf("-");
        
        type Side = "top" | "bottom" | "right" | "left";
        const side:Side = 
            splitIndex === -1 ? placement : placement.substr(0, splitIndex) as any;

        type Align = "middle" | "start" | "end";
        const align:Align = 
            splitIndex === -1 ? "middle" : placement.substr(splitIndex+1) as any;

        const targetRect = target.domRect; 
        const contentRect = content.domRect; 

        let x:number = targetRect.x;
        let y:number = targetRect.y;
        if(side === "top") {
            y -= (contentRect.height + margin);
        } else if(side === "bottom") {
            y += (targetRect.height + margin);
        } else if(side === "right") {
            x += (targetRect.width + margin);
        } else if(side === "left") {
            x -= (contentRect.width + margin);
        }

        if(align == "middle") {
            if(side === "bottom" || side === "top") {
                x = targetRect.left + ((targetRect.width - contentRect.width)/2);
            } else {
                y = targetRect.top + ((targetRect.height- contentRect.height)/2);
            }
        } else if(align == "start") {
            if(side === "bottom" || side === "top") {
                x = targetRect.left;
            } else {
                y = targetRect.top;
            }
        } else if(align == "end") {
            if(side === "bottom" || side === "top") {
                x = (targetRect.right - contentRect.width);
            } else {
                y = (targetRect.bottom - contentRect.height);
            }
        }

        if(container.valid) {
            const containerRect = container.domRect; 

            let newSide:string = "";

            if((x + contentRect.width) > containerRect.right) {
                newSide = "left";
            }
            if((y + contentRect.height) > containerRect.bottom) {
                newSide = "top";
            }
            if(x < containerRect.left) {
                newSide = "right";
            }
            if(y < containerRect.top) {
                newSide = "bottom";
            }

            if(newSide !== "") {
                _recalc(
                    `${newSide}-${align}` as Placement,
                    recurseDepth-1
                );
                return;
            }
        }

        content.withElement(element => {
            let style:CSSStyleDeclaration = element.style;

            style.setProperty('top', `${y}px`);
            style.setProperty('left', `${x}px`);
        });
    }

    const recalc = () => _recalc(opts.placement, 3);
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
