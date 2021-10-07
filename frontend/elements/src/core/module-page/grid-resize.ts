import { LitElement, html, css, customElement, property } from 'lit-element';
import { nothing } from "lit-html";
import { BgBlue } from "@elements/_styles/bg";
import { startResizer, setResizeOnStyle, setResizeOnDocumentRoot } from "@utils/resize";
import {classMap} from "lit-html/directives/class-map";
import {STAGE_PLAYER, STAGE_EDIT, STAGE_LEGACY} from "@utils/config";
import { loadAllFonts, loadFonts } from '@elements/_themes/themes';

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

        #overlay {
            position: fixed;
            top: 0;
            left: 0; 
            display: block;
            z-index: 1000;
        }

        #container {
            position: absolute;
            top: var(--y);
            left: var(--x);
            width: var(--width);
            height: var(--height);
            background-color: white;
        }

        .main {
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

        .grid {
            display: grid;

            grid-template-areas:
                        "sidebar fillLeft header fillRight"
                        "sidebar fillLeft main fillRight"
                        "sidebar fillLeft footer fillRight";
            grid-template-columns: auto 40px 1fr 40px;
            grid-template-rows: auto 1fr auto;
            height: 100%;
            width: 100%;
        }
        .grid-preview {
            display: grid;

            grid-template-areas:
                        "sidebar fillLeft header fillRight"
                        "sidebar fillLeft main fillRight"
                        "sidebar fillLeft footer fillRight";
            grid-template-columns: auto 0 1fr 0;
            grid-template-rows: auto 1fr auto;
            height: 100%;
            width: 100%;
        }

        aside {
            grid-area: sidebar;
            z-index: 2;
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

        .hidden {
            display: none;
        }

    `];
    }

    @property({ type: Boolean })
    legacy: boolean = false;

    @property({ type: Boolean })
    scrollable: boolean = false;

    @property({ type: Boolean, reflect: true })
    preview: boolean = false;

  @property({type: Boolean})
  fontsLoaded:boolean = false;

  @property()
  fontFamilies:Array<string> | undefined;

    firstUpdated() {
        const shadowRoot = this.shadowRoot as ShadowRoot;

        const sidebar = shadowRoot.querySelector("aside") as HTMLElement;
        const header = shadowRoot.querySelector("header") as HTMLElement;
        const footer = shadowRoot.querySelector("footer") as HTMLElement;

        const [_, cancelResize] = startResizer({
            stage: this.legacy ? STAGE_LEGACY 
                : this.preview ? STAGE_PLAYER : STAGE_EDIT,
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
        }, (info) => {
            setResizeOnDocumentRoot(info);
            this.dispatchEvent(new CustomEvent('module-resize', {
                detail: info
            }));
        });

        this.cancelResize = cancelResize;

        if(this.fontFamilies) {
            loadFonts(this.fontFamilies).then(() => {
                this.fontsLoaded = true;
            });
        } else {
            loadAllFonts().then(() => {
                this.fontsLoaded = true;
            });
        }
    }

    disconnectedCallback() {
        if (this.cancelResize) {
            this.cancelResize();
        }

        this.cancelResize = null;
    }

    // Define the element's template
    render() {
        const { scrollable, preview } = this;

        const scrollStyle = scrollable ? `overflow-auto` : `overflow-hidden`;

        const gridClass = classMap({
            "grid-preview": preview,
            "grid": !preview,
            hidden: !this.fontsLoaded,
        });

        const overlayClass = classMap({
            hidden: !this.fontsLoaded,
        });

        return html`
        <div class="${gridClass}">
        
            <aside id="sidebar">
                <slot name="sidebar"></slot>
            </aside>
        
            <header id="header">
                <slot name="header"></slot>
            </header>
        
            <main>
                <div id="outer">
                    <div id="container">
                        <div class="main-bg">
                            <slot name="main-bg"></slot>
                        </div>
                        <div id="main" class="main ${scrollStyle}">
                            <slot name="main"></slot>
                        </div>
                    </div>
                </div>
            </main>
        
            <footer id="footer">
                <slot name="footer"></slot>
            </footer>
        </div>
        <div id="overlay" class=${overlayClass} ><slot name="overlay"></slot></div>
    `;
    }
}
