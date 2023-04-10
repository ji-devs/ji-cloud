import { LitElement, html, css, customElement, property, state } from "lit-element";
import { actionStyles } from "./action-styles";

@customElement("jig-play-landing")
export class _ extends LitElement {
    static get styles() {
        return [
            actionStyles,
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
                    height: 100dvh;
                    width: 100vw;
                    --bottom-bar-height: 38px;
                }
                @media (min-width: 1024px) {
                    :host {
                        --bottom-bar-height: 100px;
                    }
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
                    height: calc(100% - var(--bottom-bar-height));
                }
                ::slotted([slot="message"]) {
                    position: fixed;
                    height: 100dvh;
                    width: 100vw;
                    display: grid;
                    place-content: center;
                    background-color: #ffffff;
                }
                .controls {
                    box-sizing: border-box;
                    overflow: hidden;
                    z-index: 3;
                    pointer-events: none;
                    display: grid;
                    grid-template-columns: 0px 1fr auto;
                    grid-template-rows: auto 1fr var(--bottom-bar-height);
                }
                .controls ::slotted(*),
                .controls fa-button {
                    pointer-events: all;
                }
                .sidebar {
                    grid-column: 1;
                    grid-row: 1 / -1;
                    z-index: 3;
                }
                .top-bar {
                    grid-row: 1;
                    grid-column: 1 / -1;
                    display: flex;
                    justify-content: space-between;
                    padding: 16px 28px;
                }
                .top-bar .logo {
                    height: 14px;
                }
                @media (min-width: 1024px) {
                    .top-bar .logo {
                        height: 28px;
                    }
                }
                :host([inIframe]) .top-bar .module-assist {
                    margin-right: 96px;
                }
                ::slotted([slot="module-assist"]) {
                    width: 30px;
                    height: 30px;
                    background: none;
                    border: none;
                    cursor: pointer;
                    display: inline-flex;
                    place-content: center;
                }
                .indicators {
                    grid-column: 3;
                    grid-row: 2;
                    display: grid;
                    grid-gap: 16px;
                    justify-items: end;
                    align-content: start;
                }
                @media (min-width: 1024px) {
                    .indicators {
                        padding-top: 74px;
                    }
                }
                .bottom-bar {
                    grid-row: 3;
                    grid-column: 1 / -1;
                    display: grid;
                    align-items: center;
                    justify-content: center;
                    grid-gap: 16px;
                    grid-template-columns: 62px minmax(100px, 500px) 62px;
                    padding: 0 50px;
                }
                @media (min-width: 1024px) {
                    .bottom-bar {
                        padding: 0 220px;
                    }
                }
                :host([rtl]) .bottom-bar {
                    transform: scale(-1, 1);
                }
                .bottom-bar .back {
                    justify-self: end;
                }
                .bottom-right {
                    grid-row: 3;
                    grid-column: 3;
                    display: flex;
                    align-items: center;
                }
                @media (max-width: 1023px) {
                    .bottom-right {
                        margin-right: 16px;
                        column-gap: 8px;
                        rotate: 180deg;
                        /* 15px = center of button */
                        transform-origin: calc(100% - 15px) center;
                        align-self: center;
                        transition: rotate .2s, opacity .2s;
                    }
                    :host([menuOpen]) .bottom-right {
                        rotate: 90deg;
                    }
                    .bottom-right ::slotted(*) {
                        rotate: -90deg;
                        opacity: 0;
                        pointer-events: none;
                    }
                    :host([menuOpen]) .bottom-right ::slotted(*) {
                        opacity: 1;
                        pointer-events: all;
                    }
                }
                @media (min-width: 1024px) {
                    .bottom-right {
                        margin-right: 32px;
                        column-gap: 16px;
                    }
                    .menu-button {
                        display: none;
                    }
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
            `,
        ];
    }

    @property({ type: Boolean, reflect: true })
    paused: boolean = false;

    @property({ type: Boolean, reflect: true })
    isLegacy = false;

    @property({ type: Boolean, reflect: true })
    rtl: boolean = false;

    @property({ type: Boolean, reflect: true })
    inIframe: boolean = false;

    @property({ type: Boolean, reflect: true })
    menuOpen: boolean = false;

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
                    <div class="top-bar">
                        <img-ui class="logo" path="jig/play/logo.svg"></img-ui>
                        <div class="module-assist">
                            <slot name="module-assist"></slot>
                        </div>
                    </div>
                    <div class="bottom-bar">
                        <span class="back">
                            <slot name="back"></slot>
                        </span>
                        <span class="progress">
                            <slot name="progress"></slot>
                        </span>
                        <span class="forward">
                            <slot name="forward"></slot>
                        </span>
                    </div>
                    <div class="bottom-right">
                        <!-- <slot @click=${() => this.menuOpen = false} name="full-screen"></slot>
                        <slot @click=${() => this.menuOpen = false} name="background"></slot>
                        <slot @click=${() => this.menuOpen = false} name="play-pause-button"></slot> -->
                        <slot name="full-screen"></slot>
                        <slot name="background"></slot>
                        <slot name="play-pause-button"></slot>
                        <fa-button
                            class="menu-button action middle"
                            icon="fa-solid fa-ellipsis-vertical"
                            @click=${() => this.menuOpen = !this.menuOpen}
                        ></fa-button>
                    </div>
                </div>
                <slot name="dialog"></slot>
            </main>
        `;
    }
}
