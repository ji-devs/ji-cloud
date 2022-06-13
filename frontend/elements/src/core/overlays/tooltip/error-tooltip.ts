import { LitElement, html, css, customElement, property } from "lit-element";
import { nothing } from "lit-html";
import { styleMap } from "lit-html/directives/style-map";
import {computePosition, shift, flip, arrow, offset, getScrollParents} from '@floating-ui/dom';
import "@elements/core/overlays/container";
import "@elements/core/overlays/content";
import {
    TrackerProp,
    ZLayer,
    Anchor,
    ContentAnchor,
    MoveStrategy,
} from "@elements/core/overlays/content";
import "@elements/core/buttons/icon";
import "./container";
import { Color } from "./container";

@customElement("overlay-tooltip-error")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    font-family: Poppins;
                    box-shadow: 0 3px 40px 0 rgba(0, 0, 0, 0.08);
                    display: inline-block;

                    --tooltip-bg-color: var(--light-red-1);
                    --tooltip-border-color: var(--light-red-4);
                }

                .body {
                }
                div[role="tooltip"] {
                    position: absolute;
                    display: flex;
                    gap: 16px;
                    align-items: center;
                    background-color: var(--tooltip-bg-color);
                    border: solid 3px var(--tooltip-border-color);
                    font-size: 16px;
                    color: var(--dark-gray-6);
                    border-radius: 25px;
                    padding: 12px 24px;
                }

                div[role="tooltip"]::before, div[role="tooltip"]::after {
                    content: '';
                    position: absolute;
                    border-color: background: transparent black transparent transparent;
                    width: 0;
                    height: 0;
                    border-top: 8px solid transparent;
                    border-bottom: 8px solid transparent;
                    border-right: 8px solid red;
                    transform: translateX(-33px);
                }

                div[role="tooltip"]::before {
                    border-right: 8px solid var(--tooltip-border-color);
                }

                div[role="tooltip"]::after {
                    border-right: 8px solid var(--tooltip-bg-color);
                    transform: translateX(-29px);
                }

                div[role="tooltip"] #arrow {
                    position: absolute;
                    background: var(--tooltip-bg-color);
                    width: 8px;
                    height: 8px;
                    transform: rotate(45deg);
                }
            `,
        ];
    }

    connectedCallback() {
        super.connectedCallback();
        window.addEventListener("mousedown", this.onGlobalMouseDown);
    }
    disconnectedCallback() {
        super.disconnectedCallback();
        window.removeEventListener("mousedown", this.onGlobalMouseDown);
    }

    onConfirm = () => {
        this.dispatchEvent(new Event("accept"));
    };
    onCancel = () => {
        this.dispatchEvent(new Event("close"));
    };
    onGlobalMouseDown = (evt: MouseEvent) => {
        if (
            !evt
                .composedPath()
                .includes(this.shadowRoot?.getElementById("tooltip") as any)
        ) {
            this.onCancel();
        }
    };

    @property({ type: Number })
    maxWidth: number = -1;

    //internal
    @property()
    currContentAnchor: ContentAnchor = "oppositeH";

    @property()
    currTargetAnchor: Anchor = "tr";

    //pass through
    @property()
    container: TrackerProp | undefined = window;

    @property()
    target: TrackerProp | undefined;

    @property()
    strategy: MoveStrategy = "";

    @property({ reflect: true })
    zLayer: ZLayer | undefined = "tooltip";

    @property()
    contentAnchor: ContentAnchor = "oppositeH";

    @property()
    targetAnchor: Anchor = "tr";

    @property({ type: Number })
    marginX: number = 0;

    @property({ type: Number })
    marginY: number = 0;

    @property()
    color: Color = "red";

    @property({ type: Number })
    arrowNudge: number = 0;

    updatePosition() {
        let tooltip = this.shadowRoot.querySelector("div[role='tooltip']");
        let target = document.querySelector(this.target);
        let arrowElement = this.shadowRoot.querySelector("#arrow");

        return () => {
            computePosition(target, tooltip, {
                placement: 'right-start',
                middleware: [
                    shift(),
                    offset(24),
                    flip(),
                    arrow({
                        element: arrowElement,
                        padding: 25,
                    }),
                ],
            }).then(({x, y, placement, middlewareData}) => {
                tooltip.style.left = `${x}px`;
                tooltip.style.top = `${y}px`;

                let {x: arrowX, y: arrowY} = middlewareData.arrow;

                const staticSide = {
                    top: 'bottom',
                    right: 'left',
                    bottom: 'top',
                    left: 'right',
                }[placement.split('-')[0]];

                Object.assign(arrowElement.style, {
                    left: arrowX != null ? `${arrowX}px` : '',
                    top: arrowY != null ? `${arrowY}px` : '',
                    right: '',
                    bottom: '',
                    [staticSide]: '-6px',
                });
            })
        }
    }

    firstUpdated() {
        let updatePosition = this.updatePosition();
        updatePosition();

        this.addEventListener("scroll", updatePosition);
        this.addEventListener("resize", updatePosition);

        let tooltip = this.shadowRoot.querySelector("div[role='tooltip']");
        let target = document.querySelector(this.target);

        [
            ...getScrollParents(target),
            ...getScrollParents(tooltip),
        ].forEach((element) => {
            element.addEventListener("scroll", updatePosition);
            element.addEventListener("resize", updatePosition);
        })
    }

    render() {
        const {
            container,
            target,
            strategy,
            zLayer,
            marginX,
            marginY,
            contentAnchor,
            targetAnchor,
            maxWidth,
            arrowNudge,
        } = this;

        const bodyStyles: any = {};

        if (maxWidth !== -1) {
            bodyStyles.maxWidth = `${maxWidth}px`;
        }


        return html`
            <div role="tooltip">
                <img-ui path="core/tooltips/alert.svg"></img-ui>
                <div class="body" style="${styleMap(bodyStyles)}">
                    <slot></slot>
                </div>
            </div>
        `;
    }
}
