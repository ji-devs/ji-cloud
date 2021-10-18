import { LitElement, html, css, customElement, property } from 'lit-element';
import { classMap } from "lit-html/directives/class-map";
import "@elements/core/buttons/icon";

@customElement('menu-kebab')
export class _ extends LitElement {
    static get styles() {
        return [css`
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
                padding: 14px 6px 16px 14px;
            }
            .menu-container.visible {
                display: block;
            }
            #button {
                width: 32px;
                height: 32px;
            }
        `];
    }

    buttonRef: any;

    @property({ type: Boolean })
    visible: boolean = false;

    @property({ type: Number })
    offsetVertical: number = 8; //specific to jig menu, but w/e

    @property({ type: Number })
    offsetHorizontal: number = 50; //enough for a scrollbar

    onGlobalMouseDown = (evt: MouseEvent) => {
        const path = evt.composedPath();
        if (!path.includes(this.shadowRoot?.getElementById("menu-container") as any)
            && !path.includes(this.shadowRoot?.getElementById("button") as any)
        ) {
            this.visible = false;
            this.dispatchEvent(new CustomEvent("close"));
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

                window.addEventListener("mousedown", this.onGlobalMouseDown);
            }
        }
    }

    disconnectedCallback() {
        super.disconnectedCallback();
        this.removeGlobalListener();
    }

    removeGlobalListener() {
        window.removeEventListener("mousedown", this.onGlobalMouseDown);
    }

    getMenuContainerStyle() {
        const { buttonRef, visible, offsetVertical, offsetHorizontal } = this;

        if (buttonRef == null || !visible) {
            return "display: none;";
        }

        const domRect = buttonRef.getBoundingClientRect();

        //using the `right` measurement breaks in storybook for some reason
        //maybe it's a race condition to measuring after paint
        //but anyway, we know the exact size of the button
        const { top, left } = domRect;
        return `top: ${top + offsetVertical}px; left: ${left + offsetHorizontal}px`;
    }

    private toggleVisible() {
        this.visible = !this.visible;
        const event = this.visible ? "open" : "close";
        this.dispatchEvent(new Event(event));
    }

    private blockEvent(e: Event) {
        e.stopPropagation();
    }

    render() {
        const { visible } = this;

        const menuContainerClasses = classMap({
            ["menu-container"]: true,
            visible
        });

        const menuButtonIcon = visible ? "circle-kebab-blue" : "circle-kebab-grey";

        // stop click events from getting to <jig-sidebar-module>
        return html`
            <section @click=${this.blockEvent}>
                <button-icon id="button" icon="${menuButtonIcon}" @click=${this.toggleVisible}></button-icon>
                <div id="menu-container" class="${menuContainerClasses}" style="${this.getMenuContainerStyle()}">
                    <div class="menu">
                        <slot></slot>
                    </div>
                </div>
            </section>
        `;
    }
}
