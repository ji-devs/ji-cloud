import { html, css, customElement } from "lit-element";
import { BgBlue } from "@elements/_styles/bg";

@customElement("module-page-preview")
export class _ extends BgBlue {
    private cancelResize: (() => any) | null = null;

    static get styles() {
        return [
            ...super.styles,
            css`
                :host {
                    width: 100vw;
                    height: 100vh;
                    display: block;
                    padding: 0;
                    margin: 0;
                }

                #overlay {
                    position: fixed;
                    top: 0;
                    left: 0;
                    display: block;
                    z-index: 1000;
                }

                :host main {
                    display: flex;
                    flex-direction: row;
                    justify-content: center;
                }

                header {
                    grid-area: header;
                    z-index: 1;
                }

                footer {
                    grid-area: footer;
                    z-index: 1;
                }
            `,
        ];
    }

    firstUpdated() {
        // The component which renders this expects the module-resize event.
        // However, this element doesn't require anything related to resizing,
        // so we just send a fake event.
        // Alternatively, we would need to rework the component which uses this
        // which may end up touching on a lot of other parts.
        this.dispatchEvent(
            new CustomEvent("module-resize", {
                detail: {
                    scale: -1,
                    x: -1,
                    y: -1,
                    width: -1,
                    height: -1,
                    contentX: -1,
                    contentY: -1,
                    contentWidth: -1,
                    contentHeight: -1,
                },
            })
        );
    }

    // Define the element's template
    render() {
        return html`
            <div>
                <aside id="sidebar">
                    <slot name="sidebar"></slot>
                </aside>

                <header id="header">
                    <slot name="header"></slot>
                </header>

                <main>
                    <slot name="main"></slot>
                </main>
            </div>
            <div id="overlay">
                <slot name="overlay"></slot>
            </div>
        `;
    }
}
