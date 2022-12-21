import { LitElement, html, css, customElement, property } from "lit-element";
import {
    ModuleKind,
    STR_MODULE_DISPLAY_NAME,
} from "@elements/module/_common/types";
import "@elements/core/images/ui";

const SHAKE_TIME = 1000;
const STR_DRAG_ME = "Drag me";

@customElement("jig-edit-module-card")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: inline-grid;
                }
                :host([overlayShown]) {
                    transform: perspective(1px);
                    animation: shake 0.15s linear infinite;
                }
                @keyframes shake {
                    50% {
                        transform: translateX(3px) rotate(2deg);
                    }
                    100% {
                        transform: translateX(-3px) rotate(-2deg);
                    }
                }
                section {
                    grid-row: 1;
                    grid-column: 1;
                    width: 188px;
                    height: 174px;
                    border-radius: 16px;
                    box-shadow: 0 3px 6px 0 rgba(0, 0, 0, 0.16);
                    background-color: var(--white);
                    display: grid;
                    grid-template-rows: 1fr 32px;
                    cursor: grab;
                }
                .bottom {
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    text-align: center;
                    font-size: 14px;
                    border-bottom-left-radius: 16px;
                    border-bottom-right-radius: 16px;
                    font-weight: 500;
                    font-stretch: normal;
                    font-style: normal;
                    line-height: 1.25;
                    letter-spacing: normal;
                    text-align: center;
                    color: var(--dark-gray-6);
                    background-color: var(--light-blue-2);
                    position: relative;
                }
                :host([module="cover"]) .bottom {
                    background-color: var(--light-orange-1);
                }
                /* ":host([module])" is to increase the specificity because of previous selector */
                :host([module]) .top:hover + .bottom {
                    color: white;
                    background-color: var(--dark-blue-5);
                }
                .top {
                    display: flex;
                    justify-content: center;
                    align-items: center;
                }
                ::slotted([slot=stationery]) {
                    max-width: 110px;
                    max-height: 110px;
                }
                ::slotted([slot=stationery]) {
                    cursor: grab;
                }
                .overlay {
                    display: none;
                }
                :host([overlayShown]) .overlay {
                    grid-row: 1;
                    grid-column: 1;
                    pointer-events: none;
                    display: grid;
                    place-content: center;
                    background-color: red;
                    border-radius: 16px;
                    background-color: rgba(32, 64, 163, 0.9);
                }
                .overlay p {
                    margin: 0;
                    color: #ffffff;
                    font-size: 24px;
                    font-weight: bold;
                }
                ::slotted([slot=extra]) {
                    display: inline-block;
                    position: absolute;
                    right: 0;
                    margin-right: 12px;
                }
            `,
        ];
    }

    @property({ reflect: true })
    module: ModuleKind = "memory";

    @property({ type: Boolean })
    drag: boolean = false;

    @property({ type: Boolean, reflect: true })
    private overlayShown: boolean = false;

    private showOverlay() {
        this.overlayShown = true;
        setTimeout(() => {
            this.overlayShown = false;
        }, SHAKE_TIME);
    }

    render() {
        return html`
            <section @click=${this.showOverlay}>
                <div class="top">
                    <slot name="stationery"></slot>
                </div>
                <div class="bottom">
                    ${STR_MODULE_DISPLAY_NAME[this.module]}
                    <slot name="extra"></slot>
                </div>
            </section>
            <div class="overlay">
                <p>${STR_DRAG_ME}</p>

                <img-ui path="entry/jig/arrow-drag-me.svg"></img-ui>
            </div>
        `;
    }
}
