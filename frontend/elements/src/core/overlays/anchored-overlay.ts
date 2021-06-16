import { LitElement, html, css, customElement, property, query } from 'lit-element';

export type PositionX = "left-out" | "right-out" | "left-in" | "right-in" | "center";
export type PositionY = "top-out" | "bottom-out" | "top-in" | "bottom-in" | "center";

@customElement("anchored-overlay")
export class AnchoredOverlay extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    position: relative;
                    display: inline-block;
                }
                .overlay {
                    display: none;
                    position: absolute;
                    background-color: #ffffff;
                    z-index: 1;
                }
                :host([open]) .overlay {
                    display: block;
                }
                :host([positionY=top-out]) .overlay {
                    bottom: 100%;
                }
                :host([positionY=top-in]) .overlay {
                    top: 0;
                }
                :host([positionY=bottom-out]) .overlay {
                    top: 100%;
                }
                :host([positionY=bottom-in]) .overlay {
                    bottom: 0;
                }
                :host([positionY=center]) .overlay {
                    /* from https://stackoverflow.com/a/25776315/5253155 */
                    top: 50%;
                    transform: translateY(-50%);
                }
                :host([positionX=right-out]) .overlay {
                    left: 100%;
                }
                :host([positionX=right-in]) .overlay {
                    right: 0;
                }
                :host([positionX=left-out]) .overlay {
                    right: 100%;
                }
                :host([positionX=left-in]) .overlay {
                    left: 0;
                }
                :host([positionX=center]) .overlay {
                    left: 50%;
                    transform: translateX(-50%);
                }
                :host([positionY=center][positionX=center]) .overlay {
                    /* when both are center but only one transform can be applied */
                    transform: translate(-50%, -50%);
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

    @property({ reflect: true })
    positionY: PositionY = "top-out";

    @property({ reflect: true })
    positionX: PositionX = "right-out";

    @property({ type: Boolean })
    autoClose: boolean = true;

    @property({ type: Boolean, reflect: true })
    open: boolean = false;

    render() {
        return html`
            <div class="anchor">
                <slot name="anchor"></slot>
            </div>
            <div part="overlay" class="overlay">
                <slot name="overlay"></slot>
            </div>
        `;
    }
}
