/*
 * This container is encapsulated in an element
 * because the difference between it being dragged around or not is irrelevant
 *
 * will dispatch a close event when clicked outside
 */

//amount of pixels moved after which we disable the child pointer events
const DISABLE_CHILD_POINTER_THRESHHOLD = 3;

import { LitElement, html, css, customElement, property } from "lit-element";

@customElement("drag-container")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    cursor: pointer;

                    /* make sure host doesn't cover up anything on the page */
                    height: 0;
                }

                ::slotted(*) {
                    user-drag: none;
                    user-select: none;
                    -moz-user-select: none;
                    -webkit-user-drag: none;
                    -webkit-user-select: none;
                    -ms-user-select: none;
                    touch-action: none;
                }

                :host([disableChildPointer]) ::slotted(*) {
                    pointer-events: none;
                }
            `,
        ];
    }

    @property({ type: Boolean, reflect: true })
    disableChildPointer: boolean = false;

    @property({ type: Number })
    x: number = 0;

    @property({ type: Number })
    y: number = 0;

    lastPointerX: number = 0;
    lastPointerY: number = 0;

    accumDiffX: number = 0;
    accumDiffY: number = 0;

    onThisPointerDown = (evt: PointerEvent) => {
        this.accumDiffX = 0;
        this.accumDiffY = 0;
        this.disableChildPointer = false;

        this.lastPointerX = evt.clientX;
        this.lastPointerY = evt.clientY;

        window.addEventListener("pointermove", this.onGlobalPointerMove);
        window.addEventListener("pointerup", this.onGlobalPointerUp);
    };

    onGlobalPointerMove = (evt: PointerEvent) => {
        const diffX = evt.clientX - this.lastPointerX;
        const diffY = evt.clientY - this.lastPointerY;

        this.x += diffX;
        this.y += diffY;

        this.lastPointerX = evt.clientX;
        this.lastPointerY = evt.clientY;

        this.accumDiffX += Math.abs(diffX);
        this.accumDiffY += Math.abs(diffX);

        if (
            this.accumDiffX > DISABLE_CHILD_POINTER_THRESHHOLD ||
            this.accumDiffY >= DISABLE_CHILD_POINTER_THRESHHOLD
        ) {
            this.disableChildPointer = true;
        }
    };

    onGlobalPointerDown = (evt: PointerEvent) => {
        const self = this.shadowRoot?.getElementById("section") as any;

        if (!evt.composedPath().includes(self)) {
            this.dispatchEvent(new Event("close"));
        }
    };

    onGlobalPointerUp = (_evt: PointerEvent) => {
        this.removeGlobalMoveListener();
        this.removeGlobalUpListener();
        this.disableChildPointer = false;
    };

    removeGlobalDownListener() {
        window.removeEventListener("pointerdown", this.onGlobalPointerDown);
    }

    removeGlobalMoveListener() {
        window.removeEventListener("pointermove", this.onGlobalPointerMove);
    }
    removeGlobalUpListener() {
        window.removeEventListener("pointerup", this.onGlobalPointerUp);
    }

    //lifecycle
    firstUpdated() {
        window.addEventListener("pointerdown", this.onGlobalPointerDown);
    }

    disconnectedCallback() {
        super.disconnectedCallback();
        this.removeGlobalDownListener();
        this.removeGlobalMoveListener();
        this.removeGlobalUpListener();
    }

    render() {
        const { x, y } = this;

        const style = `transform: translate(${x}px, ${y}px);`;

        return html`<section
            id="section"
            style="${style}"
            @pointerdown=${this.onThisPointerDown}
        >
            <slot></slot>
        </section>`;
    }
}
