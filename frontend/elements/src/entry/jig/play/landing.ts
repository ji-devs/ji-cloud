import { LitElement, html, css, customElement, property } from "lit-element";

@customElement("jig-play-landing")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                /*
                    z-index layers:
                    1) iframe
                    2) pause overlay
                    3) controls
                    4) dialog
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
                ::slotted([slot="message"]) {
                    position: fixed;
                    height: 100vh;
                    width: 100vw;
                    display: grid;
                    place-content: center;
                    background-color: #ffffff;
                }
                .controls {
                    display: grid;
                    grid-template-columns: 0px 1fr auto;
                    grid-template-rows: 1fr 100px;
                    box-sizing: border-box;
                }
                .controls {
                    z-index: 3;
                    pointer-events: none;
                }
                .controls ::slotted(*) {
                    pointer-events: all;
                }
                .sidebar {
                    grid-column: 1;
                    grid-row: 1 / -1;
                    z-index: 3;
                }
                .indicators {
                    grid-column: 3;
                    grid-row: 1;
                    display: grid;
                    grid-gap: 16px;
                    justify-items: end;
                    align-content: start;
                    padding-top: 74px;
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
                ::slotted(dialog-overlay) {
                    background-color: #00000080;
                }
                :host([paused]) .paused-backdrop {
                    z-index: 2;
                }
                ::slotted(dialog-overlay) {
                    z-index: 4;
                }

                /* mobile */
                @media (max-width: 1000px) {
                    :host([isLegacy]) ::slotted([slot="iframe"]){
                        height: calc(100% - 50px);
                    }
                    .controls {
                        grid-template-rows: 1fr 50px;
                    }
                    .bottom-bar {
                        grid-template-columns: 30px 1fr 30px;
                        padding: 0 80px;
                    }
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
                <slot name="message"></slot>
                <slot name="iframe"></slot>
                <div class="paused-backdrop"></div>
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
                        <slot name="play-pause-button"></slot>
                    </div>
                </div>
                <slot name="dialog"></slot>
            </main>
        `;
    }
}
