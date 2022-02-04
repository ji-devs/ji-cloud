import { LitElement, html, css, customElement } from "lit-element";

@customElement("theme-background-color")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    background-color: #fff;
                    padding: 16px;
                    box-shadow: 0 3px 6px 0 rgba(0, 0, 0, 0.16);
                    border-radius: 14px;
                    display: grid;
                    max-height: 100vh;
                    overflow: auto;
                }
                ::slotted(*) {
                    grid-area: 1 / 1;
                }
                ::slotted([slot=close]) {
                    justify-self: end;
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

    private onGlobalMouseDown = (evt: MouseEvent) => {
        if (!evt.composedPath().includes(this)) {
            this.onClose();
        }
    };

    private onClose = () => {
        this.dispatchEvent(new Event("close"));
    };

    render() {
        return html`
            <slot name="main"></slot>
            <slot name="close"></slot>
        `;
    }
}
