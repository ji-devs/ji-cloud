import { scrollbarStyles } from "@elements/_styles/scrollbar";
import {
    LitElement,
    html,
    css,
    customElement,
    property,
    query,
    PropertyValues,
} from "lit-element";
import { classMap } from "lit-html/directives/class-map";

export type PositionX =
    | "left-out"
    | "right-out"
    | "left-in"
    | "right-in"
    | "center";
export type PositionY =
    | "top-out"
    | "bottom-out"
    | "top-in"
    | "bottom-in"
    | "center";

@customElement("anchored-overlay")
export class AnchoredOverlay extends LitElement {
    static get styles() {
        return [
            scrollbarStyles,
            css`
                :host {
                    display: inline-block;
                }
                .overlay {
                    position: fixed;
                    z-index: 2;
                    overflow: auto;
                    box-sizing: border-box;
                    display: none;
                }
                :host([styled]) .overlay {
                    border-radius: 16px;
                    box-shadow: rgb(0 0 0 / 25%) 0px 3px 16px 0px;
                    background-color: #ffffff;
                }
                :host([open]) .overlay {
                    display: block;
                }
            `,
        ];
    }

    connectedCallback() {
        super.connectedCallback();
        window.addEventListener("mousedown", this.onGlobalMouseDown);
    }
    disconnectedCallback() {
        super.disconnectedCallback();
        window.removeEventListener("mousedown", this.onGlobalMouseDown);
    }
    onGlobalMouseDown = (evt: MouseEvent) => {
        if (this.open && !evt.composedPath().includes(this)) {
            if (this.autoClose) {
                this.open = false;
            }
            this.dispatchEvent(new Event("close"));
        }
    };

    @query(".overlay")
    overlay!: HTMLElement;

    @query(".anchor")
    anchor!: HTMLElement;

    @property({ reflect: true })
    positionY: PositionY = "bottom-out";

    @property({ reflect: true })
    positionX: PositionX = "center";

    @property({ type: Boolean })
    autoClose: boolean = true;

    @property({ type: Boolean })
    scrollClose: boolean = true;

    @property({ type: Boolean, reflect: true })
    open: boolean = false;

    // normally we try to stay away from styling a low level primitive like this,
    // but since doing this separately on each implementation is hard, since :part and ::slotted can't be combined, I've decided to provide this as an option here.
    // another option would be to create a new styled version that uses this implementation inside, like we do for select.
    @property({ type: Boolean, reflect: true })
    styled: boolean = false;

    firstUpdated() {
        const observer = new ResizeObserver(this.reposition);
        observer.observe(this.overlay);
        document.addEventListener("scroll", this.reposition);
    }

    updated(propertyValues: PropertyValues) {
        if (propertyValues.has("open")) this.onOpenChange();
    }

    private onOpenChange() {
        if (this.open) {
            if (this.scrollClose)
                document.addEventListener("scroll", this.reposition);
        } else {
            document.removeEventListener("scroll", this.reposition);
            this.overlay.style.removeProperty("max-height");
            this.overlay.style.removeProperty("max-width");
        }
    }

    private reposition = async() => {
        const overlayBounds = this.overlay.getBoundingClientRect();
        const anchorBounds = this.anchor.getBoundingClientRect();
        const anchorEdgeDistances = new EdgeDistances(anchorBounds);
        let positionX = this.positionX;
        let positionY = this.positionY;
        const xPlaceAt = fits({
            length: overlayBounds.width,
            spaceBefore: anchorEdgeDistances.left,
            spaceAfter: anchorEdgeDistances.right,
        });
        if(!isSameX(this.positionX, xPlaceAt.placeAt)){
            positionX = flipX(positionX);
        }
        const yPlaceAt = fits({
            length: overlayBounds.height,
            spaceBefore: anchorEdgeDistances.top,
            spaceAfter: anchorEdgeDistances.bottom,
        });
        if(!isSameY(this.positionY, yPlaceAt.placeAt)){
            positionY = flipY(positionY);
        }

        addStyles({
            positionX,
            positionY,
            scrollbar: xPlaceAt.scrollbar || yPlaceAt.scrollbar,
            anchorBounds,
            overlayBounds,
            anchorEdgeDistances: anchorEdgeDistances,
            overlay: this.overlay,
        });
    }

    render() {
        const overlayClasses = classMap({
            overlay: true,
            scrollbar: this.styled,
        });

        return html`
            <div part="anchor" class="anchor">
                <slot name="anchor"></slot>
            </div>
            <div part="overlay" class=${overlayClasses}>
                <slot name="overlay"></slot>
            </div>
        `;
    }
}

function flipX(pos: PositionX): PositionX {
    if(pos === "left-in")
        return "right-in";
    else if(pos === "left-out")
        return "right-out";
    else if(pos === "right-in")
        return "left-in";
    else if(pos === "right-out")
        return "left-out";
    return "center";
}
function flipY(pos: PositionY): PositionY {
    if(pos === "top-in")
        return "bottom-in";
    else if(pos === "top-out")
        return "bottom-out";
    else if(pos === "bottom-in")
        return "top-in";
    else if(pos === "bottom-out")
        return "top-out";
    return "center";
}


// left-out and right-in are both overflowing to the left. Same for other cases.
function isSameX(pos: PositionX, placeAt: PlaceAt): boolean {
    if (placeAt === "either")
        return true;

    if (placeAt === "before")
        return pos === "left-out" || pos === "right-in";

    return pos === "right-out" || pos === "left-in";
}
function isSameY(pos: PositionY, placeAt: PlaceAt): boolean {
    if (placeAt === "either")
        return true;

    if (placeAt === "before")
        return pos === "top-out" || pos === "bottom-in";

    return pos === "bottom-out" || pos === "top-in";
}


interface FitsArgs {
    length: number,
    spaceBefore: number,
    spaceAfter: number,
}
type PlaceAt = "either" | "before" | "after";
interface FitsResult {
    placeAt: PlaceAt;
    scrollbar: boolean;
}

function fits(args: FitsArgs): FitsResult {
    const {length, spaceBefore, spaceAfter} = args;
    let placeAt: PlaceAt;
    let scrollbar = false;

    if(length <= spaceBefore && length <= spaceAfter) {
        placeAt = "either";
    } else if(length <= spaceBefore) {
        placeAt = "before";
    } else if(length <= spaceAfter) {
        placeAt = "after";
    } else {
        scrollbar = true;
        if(spaceBefore > spaceAfter) {
            placeAt = "before";
        } else {
            placeAt = "after";
        }
    }

    return {
        placeAt,
        scrollbar
    }
}

class EdgeDistances {
    public left: number;
    public right: number;
    public top: number;
    public bottom: number;
    constructor(rect: DOMRect) {
        this.left = rect.left;
        this.right = innerWidth - rect.right;
        this.top = rect.top;
        this.bottom = innerHeight - rect.bottom;
    }
}


interface AddStylesArgs {
    positionX: PositionX;
    positionY: PositionY;
    scrollbar: boolean;
    anchorBounds: DOMRect;
    overlayBounds: DOMRect;
    anchorEdgeDistances: EdgeDistances;
    overlay: HTMLElement;
}
function addStyles(args: AddStylesArgs) {
    const { positionX, positionY, anchorBounds, overlayBounds, overlay } = args;

    let top: number;
    switch (positionY) {
        case "top-out": {
            top = anchorBounds.top - overlayBounds.height;
            break;
        }
        case "bottom-out": {
            top = anchorBounds.bottom;
            break;
        }
        case "top-in": {
            top = anchorBounds.top;
            break;
        }
        case "bottom-in": {
            top = anchorBounds.bottom - overlayBounds.height;
            break;
        }
        case "center": {
            const center = anchorBounds.height / 2 - overlayBounds.height / 2;
            top = anchorBounds.top + center;
            break;
        }
    }
    args.overlay.style.setProperty("top", `${top}px`);

    let left: number;
    switch (positionX) {
        case "left-out":
            left = anchorBounds.left - overlayBounds.width;
            break;
        case "right-out":
            left = anchorBounds.right;
            break;
        case "left-in":
            left = anchorBounds.left;
            break;
        case "right-in":
            left = anchorBounds.right - overlayBounds.width;
            break;
        case "center":
            const center = anchorBounds.width / 2 - overlayBounds.width / 2;
            left = anchorBounds.left + center;
            break;
    }
    args.overlay.style.setProperty("left", `${left}px`);

    if(args.scrollbar){
        const maxHeight = (positionY === "top-in" || positionY === "top-out") ? args.anchorEdgeDistances.top : args.anchorEdgeDistances.bottom;
        args.overlay.style.setProperty("max-height", `${maxHeight}px`);

        const maxWidth = window.innerWidth - left;
        args.overlay.style.setProperty("max-width", `${maxWidth}px`);
    }
}
