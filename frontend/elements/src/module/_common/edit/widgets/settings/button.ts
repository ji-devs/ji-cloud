import { LitElement, html, css, customElement, property } from "lit-element";
import { nothing } from "lit-html";
import { classMap } from "lit-html/directives/class-map";
import "@elements/core/images/ui";

export type Kind =
    | "attempts"
    | "card-double"
    | "card-single"
    | "cards-show-all"
    | "cards-show-some"
    | "continue-all"
    | "continue-click"
    | "continue-some"
    | "highlight"
    | "highlight-off"
    | "no-limit"
    | "n_choices"
    | "n_pairs"
    | "order"
    | "randomize"
    | "rounds"
    | "score"
    | "score-off"
    | "swap"
    | "time-limit"
    | "time-limit-off"
    | "video-captions"
    | "play-click"
    | "mute"
    | "loop"
    | "continue-automatically";

const OneImage: Set<Kind> = new Set([
    "n_choices",
    "rounds",
]);

const STR_LABEL: Record<Kind, string> = {
    "randomize": "randomize",
    "order": "ask in order",
    "no-limit": "no limit",
    "attempts": "multiple tries",
    "score": "include in final score",
    "score-off": "don't include in final score",
    "time-limit-off": "no time limit",
    "time-limit": "time limit (seconds)",
    "continue-click": "clicking next",
    "continue-all": "clicking all items",
    "continue-some": "clicking a minimum",
    "highlight": "highlight at start",
    "highlight-off": "don't highlight",
    "card-single": "double-sided",
    "card-double": "side-by-side",
    "cards-show-all": "show all",
    "cards-show-some": "show some",
    "rounds": "pages per game",
    "n_choices": "cards per page",
    "n_pairs": "pairs per game",
    "swap": "card position",
    "video-captions": "play with captions",
    "play-click": "play on click",
    "mute": "play without sound",
    "loop": "play on loop",
    "continue-automatically": "after video",
};

@customElement("module-settings-button")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    position: relative;
                    top: 0;
                    left: 0;
                    display: flex;
                    flex-direction: column;
                    align-items: center;
                }
                .icon-and-label {
                    display: flex;
                    flex-direction: column;
                    align-items: center;
                    width: 100%;
                }
                .icon {
                    position: relative;
                    top: 0;
                    left: 0;
                    cursor: pointer;
                    width: 60px;
                    height: 60px;
                }
                img-ui {
                    display: inherit;
                    width: inherit;
                    height: inherit;
                    object-fit: inherit;
                }

                .hidden {
                    display: none;
                }

                .num-circle {
                    position: absolute;
                    width: 24px;
                    height: 24px;
                    top: 45px;
                    right: -12px;
                }

                .circle {
                    background-color: #afcbf4;
                    border-radius: 50%;
                    width: 26px;
                    height: 26px;
                    text-align: center;
                    font-size: 13px;
                    display: inline-grid;
                    align-content: center;
                    transform: translate(170%, -70%);
                    color: #ffffff;
                }

                .circle.active {
                    background-color: #5893f9;
                }

                .label {
                    pointer-events: none;
                    margin-top: 10px;
                    line-height: 1.14;
                    letter-spacing: normal;
                    text-align: center;
                    color: var(--dark-gray-6);
                    max-width: 100px;
                    font-size: 13px;
                }

                :host([active]) .label {
                    font-weight: 600;
                    color: #5893f9;
                }

                /*
                    Position the bubble origin so that it's in the middle here
                    the bubble will nudge itself to the left;
                */
                .bubble {
                    display: block;
                    position: relative;
                    top: 0;
                    left: 0;
                    width: 0px;
                    height: 0px;
                    white-space: nowrap;
                    overflow: visible;
                    z-index: 1000;
                }
            `,
        ];
    }

    @property()
    kind: Kind = "attempts";

    @property()
    label?: string;

    @property({ type: Boolean, reflect: true })
    hover: boolean = false;

    @property({ type: Boolean })
    bubbleOpen: boolean = false;

    @property({ type: Boolean, reflect: true })
    active: boolean = false;

    @property({ type: Number })
    num: number = NaN;

    connectedCallback() {
        super.connectedCallback();
        window.addEventListener("mousedown", this.onGlobalMouseDown);
    }

    disconnectedCallback() {
        super.disconnectedCallback();
        window.removeEventListener("mousedown", this.onGlobalMouseDown);
    }

    onGlobalMouseDown = (evt: MouseEvent) => {
        if (this.bubbleOpen && !evt.composedPath().includes(this.shadowRoot as any)) {
            this.dispatchEvent(new Event("close"));
        }
    };


    render() {
        const { kind, hover, active, num, label } = this;

        return html`
            <div class="icon-and-label">
                <div class="icon">
                    ${renderImage(kind, hover, active)}
                    ${!isNaN(num) ? renderNumber(num, hover, active) : nothing}
                </div>
                <div class="label">${label ?? STR_LABEL[kind]}</div>
            </div>
            ${active
                ? html`<div class="bubble"><slot name="bubble"></slot></div>`
                : nothing}
            <slot></slot>
        `;
    }
}

function renderImage(kind: Kind, hover: boolean, active: boolean) {
    const isOneImage = OneImage.has(kind);

    const basePath = `module/_common/edit/widgets/sidebar/settings/icons`;

    const regularClass = classMap({
        hidden: !isOneImage && (hover || active),
    });
    const hoverClass = classMap({
        hidden: isOneImage || !hover || active,
    });

    const activeClass = classMap({
        hidden: isOneImage || !active,
    });

    return html`
        <img-ui class=${regularClass} path="${basePath}/${kind}.svg"></img-ui>
        ${!isOneImage
            ? html`
                  <img-ui
                      class=${hoverClass}
                      path="${basePath}/${kind}-hover.svg"
                  ></img-ui>
                  <img-ui
                      class=${activeClass}
                      path="${basePath}/${kind}-active.svg"
                  ></img-ui>
              `
            : nothing}
    `;
}

function renderNumber(num: number, hover: boolean, active: boolean) {
    const circleClass = classMap({
        circle: true,
        hover,
        active,
    });

    return html`
        <span class=${circleClass}>${num}</span>
    `
}
