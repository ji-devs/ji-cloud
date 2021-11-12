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
                    width: 248px;
                    height: 224px;
                    padding: 24px 0 0;
                    border-radius: 16px;
                    box-shadow: 0 3px 6px 0 rgba(0, 0, 0, 0.16);
                    background-color: var(--white);
                    display: grid;
                    grid-template-rows: 1fr 40px;
                }
                .bottom {
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    text-align: center;
                    font-size: 16px;
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
                }
                :host([module="cover"]) .bottom {
                    background-color: var(--light-orange-1);
                }
                /* ":host([module])" is to increase the specificity because of previous selector */
                :host([module]) section:hover .bottom {
                    color: white;
                    background-color: var(--dark-blue-5);
                }
                .top {
                    display: flex;
                    justify-content: center;
                    align-items: center;
                }
                img-ui {
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
            `,
        ];
    }

    onEnter() {
        this.hover = true;
    }

    onLeave() {
        this.hover = false;
    }

    @property({ reflect: true })
    module: ModuleKind = "memory";

    @property({ type: Boolean })
    drag: boolean = false;

    @property({ type: Boolean })
    hover: boolean = false;

    @property({ type: Boolean, reflect: true })
    private overlayShown: boolean = false;

    private showOverlay() {
        this.overlayShown = true;
        setTimeout(() => {
            this.overlayShown = false;
        }, SHAKE_TIME);
    }

    render() {
        const { module, drag, hover } = this;

        const iconSuffix = drag ? `-drag` : hover ? `-hover` : ``;

        const iconPath = `entry/jig/modules/large/${module}${iconSuffix}.svg`;
        return html`
            <section
                @mouseenter="${this.onEnter}"
                @mouseleave="${this.onLeave}"
                @click=${this.showOverlay}
            >
                <div class="top">
                    <img-ui path="${iconPath}"></img-ui>
                </div>
                <div class="bottom">${STR_MODULE_DISPLAY_NAME[module]}</div>
            </section>
            <div class="overlay">
                <p>${STR_DRAG_ME}</p>
                <img-ui path="entry/jig/arrow-drag-me.svg"></img-ui>
            </div>
        `;
    }
}
