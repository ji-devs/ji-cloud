import { LitElement, html, css, customElement, property } from "lit-element";

@customElement("jig-play-landing")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                /*
                    z-index layers:
                    1) iframe
                    2) controls
                    3) pause overlay
                    4) play pause button
                    5) dialog
                */
                :host {
                    display: block;
                    height: 100vh;
                    width: 100vw;
                }
                :host([isLegacy]) {
                    /* only needed for legacy since legacy doesn't cover the whole height of the page */
                    background-color: #e6f0ff;
                }
                main {
                    display: grid;
                    height: 100%;
                    width: 100%;
                }
                ::slotted([slot="iframe"]),
                .controls,
                .paused-backdrop,
                .play-pause-button-layer,
                ::slotted(dialog-overlay) {
                    grid-column: 1;
                    grid-row: 1;
                    height: 100%;
                    width: 100%;
                    border: 0;
                }
                ::slotted([slot="iframe"]) {
                    z-index: 1;
                }
                :host([isLegacy]) ::slotted([slot="iframe"]){
                    height: calc(100% - 100px);
                }
                .controls,
                .play-pause-button-layer {
                    display: grid;
                    grid-template-columns: 0px 1fr auto;
                    grid-template-rows: 1fr 100px;
                    box-sizing: border-box;
                }
                .controls {
                    z-index: 2;
                    pointer-events: none;
                }
                .controls ::slotted(*) {
                    pointer-events: all;
                }
                .sidebar {
                    grid-column: 1;
                    grid-row: 1 / -1;
                    z-index: 2;
                }
                .indicators {
                    grid-column: 3;
                    grid-row: 1;
                    display: grid;
                    grid-gap: 16px;
                    justify-items: end;
                    align-content: start;
                    padding-right: 32px;
                    padding-top: 24px;
                }
                .bottom-bar {
                    grid-row: 2;
                    grid-column: 1 / -1;
                    display: grid;
                    grid-template-columns: 62px 450px 62px;
                    align-items: center;
                    justify-content: center;
                    grid-gap: 16px;
                }
                .bottom-right {
                    grid-row: 2;
                    grid-column: 3;
                    display: flex;
                    column-gap: 40px;
                    align-items: center;
                    padding-right: 32px;
                }
                .bottom-right ::slotted([slot="background"]) {
                    margin-right: 102px;
                }
                :host([paused]) .paused-backdrop,
                ::slotted(dialog-overlay) {
                    background-color: #00000080;
                }
                :host([paused]) .paused-backdrop {
                    z-index: 3;
                }
                .play-pause-button-layer {
                    z-index: 4;
                    pointer-events: none;
                }
                .play-pause-button-layer ::slotted([slot="play-pause-button"]) {
                    pointer-events: all;
                }
                :host([paused]) .play-pause-button-layer {
                    pointer-events: all;
                }
                ::slotted(dialog-overlay) {
                    z-index: 5;
                }
            `,
        ];
    }

    @property({ type: Boolean, reflect: true })
    paused: boolean = false;

    @property({ type: Boolean, reflect: true })
    isLegacy = false;

    render() {
        return html`
            <main>
                <slot name="iframe"></slot>
                <div class="controls">
                    <div class="sidebar">
                        <slot name="sidebar"></slot>
                    </div>
                    <div class="indicators">
                        <slot name="indicators"></slot>
                    </div>
                    <div class="bottom-bar">
                        <span class="back"><slot name="back"></slot></span>
                        <span class="progress"
                            ><slot name="progress"></slot
                        ></span>
                        <span class="forward"
                            ><slot name="forward"></slot
                        ></span>
                    </div>
                    <div class="bottom-right">
                        <slot name="background"></slot>
                    </div>
                </div>
                <div class="paused-backdrop"></div>
                <div class="play-pause-button-layer">
                    <div class="bottom-right">
                        <slot name="play-pause-button"></slot>
                    </div>
                </div>
                <slot name="dialog"></slot>
            </main>
        `;
    }
}
