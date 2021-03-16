import { LitElement, html, css, customElement, property} from 'lit-element';
import {nothing} from "lit-html";
import {BgBlue} from "@elements/_styles/bg";
import { startResizer, setResizeOnStyle, setResizeOnDocumentRoot } from "@utils/resize";
import {STAGE_PLAYER, STAGE_LEGACY} from "@project-config";

@customElement('module-page-iframe')
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
            overflow: hidden;
        }
        main {
            position: absolute;
            top: 0;
            left: 0;
            width: 100%;
            height: 100%;
        }
    `];
    }

    @property({ type: Boolean })
    scrollable: boolean = false;

    @property({ type: Boolean })
    legacy: boolean = false;

    firstUpdated() {
        const shadowRoot = this.shadowRoot as ShadowRoot;

        const [_, cancelResize] = startResizer(
            {
                stage: this.legacy ? STAGE_LEGACY : STAGE_PLAYER,
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
            <main>
                <div id="outer">
                    <div id="container">
                        <div id="content" class="${scrollStyle}">
                            <slot name="main"></slot>
                        </div>
                    </div>
                </div>
            </main>
    `;
    }
}
