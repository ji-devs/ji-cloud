import { scrollbarStyles } from '@elements/_styles/scrollbar';
import { LitElement, html, css, customElement, property, query, PropertyValues } from 'lit-element';
import { nothing } from 'lit-html';
import { classMap } from 'lit-html/directives/class-map';

export type PositionX = "left-out" | "right-out" | "left-in" | "right-in" | "center";
export type PositionY = "top-out" | "bottom-out" | "top-in" | "bottom-in" | "center";

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
                    z-index: 1;
                    overflow: auto;
                    box-sizing: border-box;
                }
                :host([styled]) .overlay {
                    border-radius: 16px;
                    box-shadow: rgb(0 0 0 / 25%) 0px 3px 16px 0px;
                    background-color: #ffffff;
                }
            `
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
        if(this.open && !evt.composedPath().includes(this)) {
            if (this.autoClose) {
                this.open = false;
            }
            this.dispatchEvent(new Event("close"))
        }
    }

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

    updated(propertyValues: PropertyValues) {
        if(propertyValues.has("open"))
            this.onOpenChange();
    }

    private onOpenChange() {
        if(this.open) {
            this.positionOverlay();
            if(this.scrollClose)
                document.addEventListener("scroll", this.onScroll);
        } else {
            document.removeEventListener("scroll", this.onScroll);
        }
    }

    private onScroll = () => {
        this.open = false;
        this.dispatchEvent(new Event("close"))
    }

    private positionOverlay() {
        const thisBounds = this.getBoundingClientRect();
        const overlayBounds = this.overlay.getBoundingClientRect();

        let top: number;
        switch (this.positionY) {
            case "top-out":
                top = thisBounds.top - overlayBounds.height;
                break;
            case "bottom-out":
                top = thisBounds.bottom;
                break;
            case "top-in":
                top = thisBounds.top;
                break;
            case "bottom-in":
                top = thisBounds.bottom - overlayBounds.height;
                break;
            case "center":
                const center = (thisBounds.height / 2) - (overlayBounds.height / 2);
                top = thisBounds.top + center;
                break;
        }
        this.overlay.style.setProperty("top", `${top}px`);

        let left: number;
        switch (this.positionX) {
            case "left-out":
                left = thisBounds.left - overlayBounds.width;
                break;
            case "right-out":
                left = thisBounds.right;
                break;
            case "left-in":
                left = thisBounds.left;
                break;
            case "right-in":
                left = thisBounds.right - overlayBounds.width;
                break;
            case "center":
                const center = (thisBounds.width / 2) - (overlayBounds.width / 2);
                left = thisBounds.left + center;
                break;
        }
        this.overlay.style.setProperty("left", `${left}px`);

        let maxHeight = window.innerHeight - top;
        this.overlay.style.setProperty("max-height", `${maxHeight}px`);

        let maxWidth = window.innerWidth - top;
        this.overlay.style.setProperty("max-width", `${maxWidth}px`);
    }

    render() {
        const overlayClasses = classMap({
            "overlay": true,
            "scrollbar": this.styled
        });

        return html`
            <div part="anchor" class="anchor">
                <slot name="anchor"></slot>
            </div>
            ${ this.open ? html`
                <div part="overlay" class=${overlayClasses}>
                    <slot name="overlay"></slot>
                </div>
            ` : nothing }
        `;
    }
}


// positioning algo: ??
// if enough in preferred side: put there
// // else if enough space other side put there
// else if put at edge of screen that side
// if overlay content is a custom element it can't figure out the height/width of the element