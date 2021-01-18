import { LitElement, html, css, customElement, property } from 'lit-element';
import { nothing } from "lit-html";
import { BgBlue } from "@elements/_styles/bg";
import { startResizer, setResizeOnStyle, setResizeOnDocumentRoot } from "@utils/resize";

@customElement('module-page-grid-resize')
export class _ extends BgBlue {
    private cancelResize: (() => any) | null = null;

    static get styles() {
        return [...super.styles, css`
        :host {
            width: 100vw;
            height: 100vh;
            display: block;
            padding: 0;
            margin: 0;
        }

        #outer {
            width: 100%;
            height: 100%;
        }

        #container {
            position: absolute;
            top: var(--y);
            left: var(--x);
            width: var(--width);
            height: var(--height);
            background-color: white;
        }

        #content {
            position: absolute;
            top: var(--content-y);
            left: var(--content-x);
            width: var(--content-width);
            height: var(--content-height);
        }

        .overflow-hidden {
            overflow: hidden;
        }

        .overflow-auto {
            overflow: auto;
        }

        #grid {
            display: grid;

            grid-template-areas:
                        "sidebar header"
                        "sidebar main"
                        "sidebar footer";
            grid-template-columns: auto 1fr;
            grid-template-rows: auto 1fr auto;
            height: 100%;
            width: 100%;
        }

        aside {
            grid-area: sidebar;
            z-index: 1;
        }

        header {
            grid-area: header;
            z-index: 1;
        }

        main {
            grid-area: main;
            position: absolute;
            top: 0;
            left: 0;
            width: 100%;
            height: 100%;
        }

        footer {
            grid-area: footer;
            z-index: 1;
        }
    `];
    }

    @property({ type: Boolean })
    legacy: boolean = false;

    @property({ type: Boolean })
    scrollable: boolean = false;

    firstUpdated() {
        const shadowRoot = this.shadowRoot as ShadowRoot;

        const sidebar = shadowRoot.querySelector("aside") as HTMLElement;
        const header = shadowRoot.querySelector("header") as HTMLElement;
        const footer = shadowRoot.querySelector("footer") as HTMLElement;

        const [_, cancelResize] = startResizer({
            observeTargets: [sidebar, header, footer],
            adjustBounds: (bounds: DOMRect) => {
                const sidebarBounds = sidebar.getBoundingClientRect();
                const headerBounds = header.getBoundingClientRect();
                const footerBounds = footer.getBoundingClientRect();
                return new DOMRect(
                    sidebarBounds.width,
                    headerBounds.height,
                    bounds.width - sidebarBounds.width,
                    bounds.height - (headerBounds.height + footerBounds.height)
                );
            },
            isLegacy: this.legacy
        }, (info) => {
            setResizeOnDocumentRoot(info);
            this.dispatchEvent(new CustomEvent('module-resize', {
                detail: info
            }));
        });

        this.cancelResize = cancelResize;
    }

    disconnectedCallback() {
        if (this.cancelResize) {
            this.cancelResize();
        }

        this.cancelResize = null;
    }

    // Define the element's template
    render() {
        const { scrollable } = this;

        const scrollStyle = scrollable ? `overflow-auto` : `overflow-hidden`;

        return html`
        <div id="grid">
        
            <aside>
                <slot name="sidebar"></slot>
            </aside>
        
            <header>
                <slot name="header"></slot>
            </header>
        
            <main>
                <div id="outer">
                    <div id="container">
                        <div id="content" class="${scrollStyle}">
                            <slot name="main"></slot>
                        </div>
                    </div>
                </div>
            </main>
        
            <footer>
                <slot name="footer"></slot>
            </footer>
        </div>
    `;
    }
}