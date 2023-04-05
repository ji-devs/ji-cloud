import { html, css, customElement } from "lit-element";
import { BgBlue } from "@elements/_styles/bg";

@customElement("unit-page-preview")
export class _ extends BgBlue {
    static get styles() {
        return [
            ...super.styles,
            css`
                :host {
                    width: 100vw;
                    height: 100dvh;
                    display: block;
                    padding: 0;
                    margin: 0;
                }
                .content {
                    height: 100%;
                    width: 100%;
                    display: grid;
                    row-gap: 10px;
                    grid-template-rows: auto 1fr;
                }
                :host main {
                    display: flex;
                    flex-direction: row;
                    justify-content: center;
                }
                #overlay {
                    position: fixed;
                    top: 0;
                    left: 0;
                    display: block;
                    z-index: 1000;
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
            <div class="content">
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
