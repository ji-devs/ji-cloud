import {
    LitElement,
    html,
    css,
    customElement,
    property,
    query,
} from "lit-element";

@customElement("dialog-overlay")
export class DialogOverlay extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: none;
                    position: fixed;
                    z-index: 1;
                    top: 0%;
                    left: 0%;
                    height: 100vh;
                    width: 100vw;
                    place-content: center;
                }
                :host([open]) {
                    display: grid;
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
        if (this.open && !evt.composedPath().includes(this.overlay)) {
            if (this.autoClose) {
                this.open = false;
            }
            this.dispatchEvent(new Event("close"));
        }
    };

    @property({ type: Boolean })
    autoClose: boolean = true;

    @property({ type: Boolean, reflect: true })
    open: boolean = false;

    @query(".overlay")
    overlay!: HTMLElement;

    render() {
        return html`
            <div part="overlay" class="overlay">
                <slot></slot>
            </div>
        `;
    }
}
