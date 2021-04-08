import { LitElement, html, css, customElement, property } from 'lit-element';

export type PositionX = "left-out" | "right-out" | "left-in" | "right-in";
export type PositionY = "top-out" | "bottom-out" | "top-in" | "bottom-in";

@customElement("anchored-overlay")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    position: relative;
                    display: inline-block;
                }
                :host([backdrop]) .backdrop {
                    display: none;
                    position: fixed;
                    top: 0;
                    left: 0;
                    height: 100vh;
                    width: 100vw;
                }
                .overlay {
                    display: none;
                    position: absolute;
                    background-color: #ffffff;
                }
                :host([open]) .backdrop, :host([open]) .overlay {
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
            `
        ];
    }

    @property({ reflect: true })
    positionY: PositionY = "top-out";

    @property({ reflect: true })
    positionX: PositionX = "right-out";

    @property({ type: Boolean, reflect: true })
    backdrop: boolean = true;

    @property()
    backdropColor = "#00000020";

    @property({ type: Boolean })
    backdropClose: boolean = true;

    @property({ type: Boolean, reflect: true })
    open: boolean = false;

    private backdropClick() {
        if (this.backdropClose) {
            this.open = false;
            this.dispatchEvent(new CustomEvent("close"));
        }
    }

    render() {
        return html`
            <style>
                .backdrop {
                    background-color: ${this.backdropColor};
                }
            </style>
            <div class="anchor">
                <slot name="anchor"></slot>
            </div>
            <div @click="${() => this.backdropClick()}" class="backdrop"></div>
            <div part="overlay" class="overlay">
                <slot name="overlay"></slot>
            </div>
        `;
    }
}
