import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/buttons/icon";
import "@elements/core/overlays/anchored-overlay";

@customElement("menu-kebab")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                .menu-container {
                    padding: 14px;
                }
                .menu {
                    display: flex;
                    flex-direction: column;
                    grid-gap: 10px;
                }
                button-icon {
                    width: 30px;
                    height: 30px;
                }
            `,
        ];
    }

    buttonRef: any;

    @property({ type: Boolean })
    visible: boolean = false;

    @property({ type: Number })
    offsetVertical: number = 8; //specific to jig menu, but w/e

    @property({ type: Number })
    offsetHorizontal: number = 50; //enough for a scrollbar

    /// Work-around property so that implementations of kebab which use OverlayHandle and the
    /// <menu-items /> element can make use of menu-items positioning instead of letting this
    /// element do work and potentially causing scrolling issues on views which already have
    /// positioning such as module/edit views.
    @property({ type: Boolean })
    positioningEnabled: boolean = true;

    /// Whenever this element needs to be used with some other container outside of this element,
    /// the user can set this property so that clicking in it doesn't dispatch any events.
    @property()
    customContainer?: HTMLElement;

    /// Set the event listener so that it can be reliably removed later.
    eventListener: any;

    onGlobalMouseDown() {
        let self = this;

        // We need to memoize `this` so that it is available whenever the event fires. Without this
        // change `customContainer` is always undefined.
        return (evt: MouseEvent) => {
            const path = evt.composedPath();

            // When using customContainer we aren't gauranteed that the menu item is inside this
            // elements hierarchy. For example, if using OverlayHandle to render the menu items,
            // the rendered content is outside this element.
            let canDispatch = true;
            if (self.customContainer) {
                let items = self.customContainer.querySelectorAll('menu-line');
                items.forEach(item => {
                    if (path.includes(item)) {
                        canDispatch = false;
                    }
                })
            }

            if (
                canDispatch
                && !path.includes(
                    self.shadowRoot?.getElementById("menu-container") as any
                )
                && !path.includes(self.shadowRoot?.getElementById("button") as any)
            ) {
                self.visible = false;
                self.dispatchEvent(new CustomEvent("close"));
            }
        }
    }

    firstUpdated(_changed: any) {
        this.buttonRef = this.shadowRoot?.getElementById("button");
        this.requestUpdate();
    }

    updated(changed: any) {
        if (typeof changed.get("visible") === "boolean") {
            const { visible } = this;
            this.removeGlobalListener();
            if (visible) {
                this.eventListener = this.onGlobalMouseDown();
                window.addEventListener("mousedown", this.eventListener);
            }
        }
    }

    disconnectedCallback() {
        super.disconnectedCallback();
        this.removeGlobalListener();
    }

    removeGlobalListener() {
        if (this.eventListener) {
            window.removeEventListener("mousedown", this.eventListener);
        }
    }

    private toggleVisible() {
        this.visible = !this.visible;
        const event = this.visible ? "open" : "close";
        this.dispatchEvent(new Event(event));
    }

    render() {
        const menuButtonIcon = this.visible
            ? "circle-kebab-blue"
            : "circle-kebab-grey";

        return html`
            <anchored-overlay
                positionX="right-out"
                positionY="top-in"
                ?open="${this.visible}"
                styled
            >
                <button-icon
                    icon="${menuButtonIcon}"
                    slot="anchor"
                    @click=${this.toggleVisible}
                ></button-icon>
                <div slot="overlay" class="menu-container" id="menu-container">
                    <div class="menu">
                        <slot></slot>
                    </div>
                </div>
            </anchored-overlay>
        `;
    }
}
