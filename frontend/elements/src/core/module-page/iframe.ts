import { html, css, customElement, property } from "lit-element";
import { BgBlue } from "@elements/_styles/bg";
import { startResizer, setResizeOnDocumentRoot } from "@utils/resize";
import { STAGE_PLAYER, STAGE_LEGACY } from "@utils/config";
import { loadAllFonts, loadFonts } from "@elements/_themes/themes";
import { classMap } from "lit-html/directives/class-map";

@customElement("module-page-iframe")
export class _ extends BgBlue {
    private cancelResize: (() => any) | null = null;

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

                #overlay {
                    position: fixed;
                    top: 0;
                    left: 0;
                    display: block;
                    z-index: 1000;
                }

                #outer {
                    width: 100%;
                    height: 100%;
                }

                #content {
                    background-color: white;
                    position: relative;
                    max-height: 100%;
                    max-width: 100%;
                    top: 50%;
                    left: 50%;
                    transform: translate(-50%, -50%);
                    overflow: hidden;
                    aspect-ratio: 16 / 9;
                    display: grid;
                }
                :host([legacy]) #content {
                    aspect-ratio: 4 / 3;
                }
                main {
                    position: absolute;
                    top: 0;
                    left: 0;
                    width: 100%;
                    height: 100%;
                }
                .hidden {
                    display: none;
                }
            `,
        ];
    }

    @property({ type: Boolean })
    scrollable: boolean = false;

    @property({ type: Boolean, reflect: true })
    legacy: boolean = false;

    @property({ type: Boolean })
    fontsLoaded: boolean = false;

    @property()
    fontFamilies: Array<string> | undefined;

    firstUpdated() {
        const shadowRoot = this.shadowRoot as ShadowRoot;
        const content = shadowRoot.querySelector("#content") as HTMLElement;

        const [_, cancelResize] = startResizer(
            {
                stage: this.legacy ? STAGE_LEGACY : STAGE_PLAYER,
                canvas: content,
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

    // Define the element's template
    render() {
        const { scrollable } = this;

        const scrollStyle = scrollable ? `overflow-auto` : `overflow-hidden`;

        const classes = classMap({ hidden: !this.fontsLoaded });

        return html`
            <main class=${classes}>
                <div id="outer">
                    <div id="content" class="${scrollStyle}">
                        <slot name="main"></slot>
                    </div>
                </div>
            </main>
            <div id="overlay" class=${classes}>
                <slot name="overlay"></slot>
            </div>
        `;
    }
}
