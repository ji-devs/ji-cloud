import { LitElement, html, css, customElement, property } from 'lit-element';
import {nothing} from "lit-html";
//these are used for calculations and set statically for CSS
//making them properties would be a nice improvement
const ARROW_SIZE = 24;


//TODO - draw the arrow/container manually as svg
@customElement("tooltip-base")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    position: fixed;
                    left: 0;
                    top: 0;
                    display: inline-block;
                    z-index: 1000;
                    /*box-shadow: 0 3px 40px 0 rgba(0, 0, 0, 0.08);*/
                }

                :host([rounded]) {
                    border-radius: 25rem;
                }
                :host([color="green"]) {
                    border: solid 2px #4bb972;
                    background-color: var(--main-green);
                }
                :host([color="beige"]) {
                    border: solid 2px var(--light-orange-2);
                    background-color: var(--light-orange-1);
                }
                :host([color="red"]) {
                    background-color: var(--light-red-1);
                }

                
                .content {
                    padding: 24rem;
                }
                #arrow {
                    position: absolute;
                    left: 0; top: 0;
                    width: ${css`${ARROW_SIZE}rem`}; 
                    height: ${css`${ARROW_SIZE}rem`}; 
                    background: inherit;
                }

            `
        ];
    }

    /*
    createRenderRoot() {
      return this;
      }
     */
    instance:TooltipInstance | undefined;

    firstUpdated(_changed:any) {
        this.bindInstance();
    }

    updated(changed:any) {
        if(typeof changed.get("target") === "boolean") {
            this.bindInstance();
        }
    }

    bindInstance= () => {
        if(this.target) {
            this.killInstance();

            const target = typeof this.target === "string" ? document.getElementById(this.target) // todo recurse for shadow dom?
                : typeof this.target === "function" ? this.target()
                : targetIsDomRect(this.target) ? this.target 
                : this.target;

            if(!target) {
                throw new Error("invalid tooltip target!");
            }

            this.instance = createInstance({
                target,
                tooltip: this,
                placement: this.placement,
                margin: this.margin,
                moveStrategy: this.moveStrategy,
                container: this.container,
                arrow: {
                    element: this.shadowRoot?.getElementById("arrow") as Element,
                    offset: this.arrowOffset,
                }
            });
        } else {
            console.info("no tooltip target set in prop");
        }
    }

    killInstance = () => {
        if(this.instance != undefined) {
            this.instance.destroy();
            this.instance = undefined;
        }
    }

    disconnectedCallback() {
        this.killInstance();
    }


    @property()
    container:Element | Window = window;

    @property()
    moveStrategy:MoveStrategy = "";

    @property({type: Boolean, reflect: true})
    rounded:boolean = false;

    @property({reflect: true})
    color:COLOR = "beige";

    @property()
    target:ElementTarget | undefined;

    @property()
    placement:Placement = "left";

    @property({type: Number})
    margin:number = 0;

    @property({type: Number})
    arrowOffset:number = 0;

    @property({type: Boolean})
    closed:boolean = false;

    render() {
        const {closed} = this;

        if(closed) {
            return nothing;
        }
        return html`
            <div class="content"><slot></slot></div>
            <div id="arrow"></div>
        `;
    }
}

function targetIsDomRect(target:any):boolean {
    return typeof target.x === "number" 
    && typeof target.y === "number"
    && typeof target.width === "number"
    && typeof target.height === "number";
}

function targetIsElement(target:any):boolean {
    //TODO - make this better... instanceof?
    return !targetIsDomRect(target);
}
function getTargetDomRect(target: ElementTarget):DOMRect {
    if(targetIsDomRect(target)) {
        return target as DOMRect;
    } else {
        //TODO - handle other target types
        return (target as Element).getBoundingClientRect();
    }
}

function createInstance(opts:Opts):TooltipInstance {
    let lastTargetRect:DOMRect | undefined;
    const _recalc = (opts:Opts, recurseDepth: number) => {
        if(recurseDepth === 0) {
            return;
        }
        const {target, tooltip, placement, container, arrow} = opts;

        const splitIndex = placement.indexOf("-");
        
        type Side = "top" | "bottom" | "right" | "left";
        const side:Side = 
            splitIndex === -1 ? placement : placement.substr(0, splitIndex) as any;

        type Align = "middle" | "start" | "end";
        const align:Align = 
            splitIndex === -1 ? "middle" : placement.substr(splitIndex+1) as any;

        const targetRect = getTargetDomRect(target);
        //can help mitigate resize vs. move
        //but very confusing 
        //lastTargetRect = targetRect; //why not
        const tooltipRect = tooltip.getBoundingClientRect();


        //cheat the margin to account for arrow size
        const margin = opts.margin + ARROW_SIZE;

        let x:number = targetRect.x;
        let y:number = targetRect.y;
        if(side === "top") {
            y -= (tooltipRect.height + margin);
        } else if(side === "bottom") {
            y += (targetRect.height + margin);
        } else if(side === "right") {
            x += (targetRect.width + margin);
        } else if(side === "left") {
            x -= (tooltipRect.width + margin);
        }

        if(align == "middle") {
            if(side === "bottom" || side === "top") {
                x = targetRect.left + ((targetRect.width - tooltipRect.width)/2);
            } else {
                y = targetRect.top + ((targetRect.height- tooltipRect.height)/2);
            }
        } else if(align == "start") {
            if(side === "bottom" || side === "top") {
                x = targetRect.left;
            } else {
                y = targetRect.top;
            }
        } else if(align == "end") {
            if(side === "bottom" || side === "top") {
                x = (targetRect.right - tooltipRect.width);
            } else {
                y = (targetRect.bottom - tooltipRect.height);
            }
        }

        //TODO - handle all the edge cases
        //nudge if it goes outside the container
        //use the margin regardless of the axis, why not
        const containerRect = !container || container === window
            ? new DOMRect(0, 0, window.innerWidth, window.innerHeight)
            : (container as Element).getBoundingClientRect();

        let newSide:string = "";

        if((x + tooltipRect.width) > containerRect.right) {
            newSide = "left";
        }
        if((y + tooltipRect.height) > containerRect.bottom) {
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
                {
                    ...opts,
                    placement: `${newSide}-${align}` as Placement
                },
                recurseDepth-1
            );
            return;
        }

        let style:CSSStyleDeclaration = (tooltip as any).style;

        style.setProperty('top', `${y}px`);
        style.setProperty('left', `${x}px`);


        //Now the tooltip itself is properly positioned, accounting for drift
        //Place the arrow...

        style = (arrow.element as any).style;

        const halfHeight = ARROW_SIZE/2;
        const halfWidth = ARROW_SIZE/2;

        //get the piece of the square which will stick out after rotation
        //see https://stackoverflow.com/questions/57619285/calculate-how-much-smaller-a-square-would-have-to-be-to-fit-after-rotated-45-deg
        const rot = 45;
        const diff = Math.sin(rot * (Math.PI / 180)) * halfWidth;
            
        const offset = align === "middle"
            ? arrow.offset 
            : arrow.offset + (diff / 2);

        if(side === "top") {
            y = tooltipRect.height - halfHeight;
        } else if(side === "bottom") {
            y = -halfHeight;
        } else if(side === "right") {
            x = -halfWidth;
        } else if(side === "left") {
            x = tooltipRect.width - halfWidth;
        }

        if(align == "middle") {
            if(side === "bottom" || side === "top") {
                x = (tooltipRect.width / 2) - (halfWidth + offset); 
            } else {
                y = (tooltipRect.height / 2) - (halfHeight + offset); 
            }
        } else if(align == "start") {
            if(side === "bottom" || side === "top") {
                x = offset;
            } else {
                y = offset; 
            }
        } else if(align == "end") {
            if(side === "bottom" || side === "top") {
                x = tooltipRect.width - (ARROW_SIZE + offset); 
            } else {
                y = tooltipRect.height - (ARROW_SIZE + offset); 
            }
        }

        style.transform = `translate(${x}px, ${y}px) rotate(45deg)`;
        style.transformOrigin = "center center";
    }

    const recalc = () => _recalc(opts, 3);
    // @ts-ignore
    const observer = new ResizeObserver(recalc);
    if(targetIsElement(opts.target)) {
        observer.observe(opts.target as Element);
    }
    observer.observe(opts.tooltip);


    //very inefficient, but ResizeObserver doesn't take this into account
    let rafId:number | undefined;
    const {moveStrategy, tooltip} = opts;
    if(moveStrategy !== "") {
        const checkPosition = () => {

            const targetRect = getTargetDomRect(opts.target);

            if(lastTargetRect !== undefined) {
                if(targetRect.x !== lastTargetRect.x || targetRect.y !== lastTargetRect.y) {
                    if(moveStrategy === "track") {
                        recalc();
                    } else if(moveStrategy === "destroy") {
                        destroy();
                        (tooltip as any).closed = true;
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

interface Opts {
    target: Element | DOMRect,
    tooltip: Element,
    placement: Placement,
    margin: number,
    container: Element | Window,
    moveStrategy: MoveStrategy,
    arrow: Arrow
}

export type MoveStrategy = "" | "destroy" | "track";

interface Arrow {
    offset: number,
    element: Element
}

interface TooltipInstance {
    destroy: () => any
}

export type ElementTarget = Element | string | (() => Element) | DOMRect;

export type COLOR = "beige" | "red" | "green";

//match it in tooltip/types.rs on the rust side
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
