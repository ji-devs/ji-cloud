import { LitElement, html, css, customElement, property } from "lit-element";
import { classMap } from "lit-html/directives/class-map";
import "@elements/core/buttons/icon";

@customElement("menu-kebab")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                .menu-container {
                    display: none;
                    position: fixed;
                    top: 0;
                    left: 0;
                    border-radius: 8px;
                    -webkit-backdrop-filter: blur(30px);
                    backdrop-filter: blur(30px);
                    box-shadow: 0 3px 16px 0 rgba(0, 0, 0, 0.2);
                    background-color: var(--white);
                    padding: 14px;
                }
                .menu-container.visible {
                    display: block;
                }
                #button {
                    width: 32px;
                    height: 32px;
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

    getMenuContainerStyle() {
        if (this.positioningEnabled) {
            const { buttonRef, visible, offsetVertical, offsetHorizontal } = this;

            if (buttonRef == null || !visible) {
                return "display: none;";
            }

            const domRect = buttonRef.getBoundingClientRect();

            //using the `right` measurement breaks in storybook for some reason
            //maybe it's a race condition to measuring after paint
            //but anyway, we know the exact size of the button
            const { top, left } = domRect;
            return `top: ${top + offsetVertical}px; left: ${
                left + offsetHorizontal
            }px`;
        } else {
            return "display: none;";
        }
    }

    private toggleVisible() {
        this.visible = !this.visible;
        const event = this.visible ? "open" : "close";
        this.dispatchEvent(new Event(event));
    }

    render() {
        const { visible } = this;

        const menuContainerClasses = classMap({
            ["menu-container"]: true,
            visible,
        });

        const menuButtonIcon = visible
            ? "circle-kebab-blue"
            : "circle-kebab-grey";

        return html`
            <section>
                <button-icon
                    id="button"
                    icon="${menuButtonIcon}"
                    @click=${this.toggleVisible}
                ></button-icon>
                <div
                    id="menu-container"
                    class="${menuContainerClasses}"
                    style="${this.getMenuContainerStyle()}"
                >
                    <div class="menu">
                        <slot></slot>
                    </div>
                </div>
            </section>
        `;
    }
}
