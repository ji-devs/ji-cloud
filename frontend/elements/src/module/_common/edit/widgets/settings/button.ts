import {
    LitElement,
    html,
    svg,
    css,
    customElement,
    property,
} from "lit-element";
import { nothing } from "lit-html";
import { classMap } from "lit-html/directives/class-map";
import "@elements/core/images/ui";

export type Kind =
    | "attempts"
    | "card-double"
    | "card-single"
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
    | "autoplay"
    | "mute"
    | "loop"
    | "continue-next-activity";

const OneImage: Set<Kind> = new Set([
    "n_choices",
    "n_pairs",
    "order",
    "rounds",
]);

const STR_LABEL: Record<Kind, string> = {
    "randomize": "randomize",
    "order": "ask in order",
    "no-limit": "no limit",
    "attempts": "multiple tries",
    "score": "include in\nfinal score",
    "score-off": "don't include in\nfinal score",
    "time-limit-off": "no time limit",
    "time-limit": "time limit",
    "continue-click": "clicking on continue",
    "continue-all": "clicking all items",
    "continue-some": "clicking\na minimum",
    "highlight": "highlight at start",
    "highlight-off": "don't highlight",
    "card-single": "double sided",
    "card-double": "side by side",
    "rounds": "pages per game",
    "n_choices": "cards per page",
    "n_pairs": "cards per page",
    "swap": "card position",
    "video-captions": "play with\ncaptions",
    "autoplay": "start automatically",
    "mute": "play without sound",
    "loop": "play on loop",
    "continue-next-activity": "continue to\nnext activity\nautomatically",
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
                    width: 64px;
                    height: 64px;
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
                    fill: #afcbf4;
                }

                .circle.active {
                    fill: #5893f9;
                }

                .label {
                    pointer-events: none;
                    width: 126px;
                    margin-top: 8px;
                    white-space: pre-wrap;
                    line-height: 1.14;
                    letter-spacing: normal;
                    text-align: center;
                    color: var(--dark-gray-6);
                    max-width: 100px;
                    font-size: 13px;
                }
                @media (min-width: 1920px) {
                    .label {
                        font-size: 14px;
                    }
                }

                /* Position the bubble origin so that it's in the middle here
		the bubble will nudge itself to the left;
		*/
                .bubble {
                    display: none;
                    position: relative;
                    top: 0;
                    left: 0;
                    width: 0px;
                    height: 0px;
                    white-space: nowrap;
                    overflow: visible;
                    z-index: 1000;
                }

                :host([hover]) .bubble {
                    display: block;
                }
            `,
        ];
    }

    @property()
    kind: Kind = "attempts";

    @property({ type: Boolean, reflect: true })
    hover: boolean = false;

    @property({ type: Boolean })
    active: boolean = false;

    @property({ type: Number })
    num: number = NaN;

    connectedCallback() {
        super.connectedCallback();

        this.addEventListener("mouseenter", this.onMouseEnter);
        this.addEventListener("mouseleave", this.onMouseLeave);
    }

    disconnectedCallback() {
        super.disconnectedCallback();

        this.removeEventListener("mouseenter", this.onMouseEnter);
        this.removeEventListener("mouseleave", this.onMouseLeave);
    }

    onMouseEnter() {
        this.hover = true;
    }

    onMouseLeave() {
        this.hover = false;
    }

    render() {
        const { kind, hover, active, num } = this;

        return html`
            <div class="icon-and-label">
                <div class="icon">
                    ${renderImage(kind, hover, active)}
                    ${!isNaN(num) ? renderNumber(num, hover, active) : nothing}
                </div>
                <div class="label">${STR_LABEL[kind]}</div>
            </div>
            <div class="bubble"><slot name="bubble"></slot></div>
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
    return svg`
		<svg class="num-circle" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24">
		<g id="Group_17881" data-name="Group 17881" transform="translate(-55 -47)">
		<circle class=${circleClass} id="Ellipse_770" data-name="Ellipse 770" cx="12" cy="12" r="12" transform="translate(55 47)" />
		<text id="_2" data-name="2" transform="translate(67 64)" fill="#fff" font-size="14" font-family="Poppins-ExtraBold, Poppins" font-weight="800"><tspan x="-3.983" y="0">${num}</tspan></text>
		</g>
		</svg>
	`;
}
