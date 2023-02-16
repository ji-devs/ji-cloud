import { html, css, customElement, property } from "lit-element";
import { BgBlue } from "@elements/_styles/bg";
import { startResizer, setResizeOnDocumentRoot } from "@utils/resize";
import { STAGE_PLAYER, STAGE_EDIT, STAGE_LEGACY } from "@utils/config";
import { loadAllFonts, loadFonts } from "@elements/_themes/themes";

@customElement("module-page-grid-resize")
export class _ extends BgBlue {
    private cancelResize: (() => any) | null = null;

    static get styles() {
        return [
            ...super.styles,
            css`
                :host {
                    width: 100vw;
                    height: 100svh;
                    display: block;
                    padding: 0;
                    margin: 0;
                }
                :host(:not([fontsLoaded])) {
                    display: none;
                }

                .grid {
                    display: grid;
                    grid-template-columns: auto 1fr;
                    width: 100%;
                    height: 100%;
                }

                aside {
                    overflow: auto;
                    box-shadow: 0 3px 6px 0 rgba(0, 0, 0, 0.16);
                }

                main {
                    padding: 10px;
                    box-sizing: border-box;
                    display: grid;
                    grid-template-rows: auto minmax(0, 1fr) auto;
                    gap: 6px;
                    height: 100svh;
                }
                .canvas {
                    max-height: 100%;
                    max-width: 100%;
                    aspect-ratio: 16 / 9;
                    position: relative;
                    top: 50%;
                    left: 50%;
                    transform: translate(-50%, -50%);
                    background-color: white;
                    display: grid;

                    overflow: hidden;
                }
                :host([scrollable]) .canvas {
                    overflow: auto;
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

    @property({ type: Boolean })
    legacy: boolean = false;

    @property({ type: Boolean, reflect: true })
    scrollable: boolean = false;

    @property({ type: Boolean, reflect: true })
    preview: boolean = false;

    @property({ type: Boolean, reflect: true })
    fontsLoaded: boolean = false;

    @property()
    fontFamilies: Array<string> | undefined;

    firstUpdated() {
        const shadowRoot = this.shadowRoot as ShadowRoot;
        const canvas = shadowRoot.querySelector("#canvas") as HTMLElement;

        console.log(STAGE_EDIT);

        const [_, cancelResize] = startResizer(
            {
                stage: this.legacy
                    ? STAGE_LEGACY
                    : this.preview
                    ? STAGE_PLAYER
                    : STAGE_EDIT,
                canvas,
            },
            (info) => {
                setResizeOnDocumentRoot(info);
                this.dispatchEvent(
                    new CustomEvent("module-resize", {
                        detail: info,
                    })
                );
            }
        );

        this.cancelResize = cancelResize;

        if (this.fontFamilies) {
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

    render() {
        return html`
            <div class="grid">
                <aside>
                    <slot name="sidebar"></slot>
                </aside>

                <main>
                    <header>
                        <slot name="header"></slot>
                    </header>

                    <div id="canvas" class="canvas">
                        <slot name="main"></slot>
                    </div>

                    <footer>
                        <slot name="footer"></slot>
                    </footer>
                </main>
            </div>
            <div id="overlay">
                <slot name="overlay"></slot>
            </div>
        `;
    }
}
