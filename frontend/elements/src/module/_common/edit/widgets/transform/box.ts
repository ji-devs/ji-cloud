import {
    LitElement,
    svg,
    html,
    css,
    customElement,
    property,
} from "lit-element";
import { nothing } from "lit-html";

export type ResizeLevel = "full" | "keep-aspect-ratio" | "none";

type DotPos = "tl" | "t" | "tr" | "l" | "r" | "bl" | "b" | "br";

const dotIds: Array<DotPos> = ["tl", "t", "tr", "l", "r", "bl", "b", "br"];

const MENU_BUTTON_RADIUS = 32 / 2;
const DOT_RADIUS = 10;

//If changing this, also update the CSS for .rectLine
const RECT_STROKE_SIZE = 3;

const ROT_LINE_DISTANCE = 30;
const ROT_BUTTON_SIZE = 64;

//If changing this, also update the CSS for #rotLine
const ROT_STROKE_SIZE = 2;

@customElement("transform-box")
export class TransformBox extends LitElement {
    static get styles() {
        return [
            css`
                svg,
                img-ui,
                #menu-btn {
                    position: absolute;
                    top: 0;
                    left: 0;
                }

                /* TODO - figure out how to make just the transform itself
                appear above all
                not the child contents

                currently breaks for text
            */
                :host {
                    position: absolute;
                    top: 0;
                    left: 0;
                    z-index: 1;
                }

                #rotLine {
                    stroke: var(--main-blue);
                    stroke-width: 2;
                }

                #rotButton {
                    cursor: pointer;
                }

                .rectLine {
                    stroke: var(--main-blue);
                    stroke-width: 3;
                    stroke-dasharray: 4px;
                }
                #fillRect {
                    cursor: move;
                    fill-opacity: 0;
                }

                .dot {
                    position: absolute;
                    top: 0;
                    left: 0;
                    fill: var(--main-blue);
                }

                .dot.tl {
                    cursor: se-resize;
                }
                .dot.tr {
                    cursor: ne-resize;
                }
                .dot.bl {
                    cursor: sw-resize;
                }
                .dot.br {
                    cursor: se-resize;
                }

                .dot.t,
                .dot.b {
                    cursor: n-resize;
                }
                .dot.l,
                .dot.r {
                    cursor: e-resize;
                }
            `,
        ];
    }

    firstUpdated(_changedProperties: any) {
        this.updateMenuButtonLocation();
    }

    updated(changed: any) {
        if (typeof changed.get("isTransforming") === "boolean") {
            const { isTransforming } = this;
            if (isTransforming) {
                this.updateMenuButtonLocation();
            }
        }
    }

    @property()
    menuButtonDot: DotPos = "tr";

    @property({ type: Boolean })
    buttonHack: boolean = false;

    private timeoutId: any = null;

    updateMenuButtonLocation = () => {
        //setting a small timeout as a hack to fight browser measuring delay
        //delay is in ms
        this.buttonHack = true;
        const HACK_DELAY = 1000;

        if (this.timeoutId !== null) {
            clearTimeout(this.timeoutId);
        }

        this.timeoutId = setTimeout(() => {
            type ToBeat = { id: DotPos; domRect: DOMRect };
            let toBeat: ToBeat = null as unknown as ToBeat;

            dotIds.forEach((id, index) => {
                const ref = this.shadowRoot?.getElementById(`dot-${id}`);
                if (ref != null) {
                    const domRect = ref.getBoundingClientRect();
                    if (index === 0) {
                        toBeat = { id, domRect };
                    } else {
                        if (domRect.y === toBeat.domRect.y) {
                            if (domRect.x > toBeat.domRect.x) {
                                toBeat = { id, domRect };
                            }
                        } else if (domRect.y < toBeat.domRect.y) {
                            toBeat = { id, domRect };
                        }
                    }
                }
            });

            if (toBeat != null) {
                this.menuButtonDot = toBeat.id;
            }
            this.buttonHack = false;
        }, HACK_DELAY);
    };

    getDotBounds = () => {
        return dotIds.map((id) =>
            this.shadowRoot
                ?.getElementById(`dot-${id}`)
                ?.getBoundingClientRect()
        );
    };

    onResizeStart(pos: DotPos, evt: MouseEvent) {
        this.dispatchEvent(
            new CustomEvent("transform-resize-start", {
                detail: {
                    pos,
                    x: evt.clientX,
                    y: evt.clientY,
                },
            })
        );
    }

    onMoveStart(evt: MouseEvent) {
        this.dispatchEvent(
            new CustomEvent("transform-move-start", {
                detail: {
                    x: evt.clientX,
                    y: evt.clientY,
                },
            })
        );
    }

    onRotateStart(evt: MouseEvent) {
        this.dispatchEvent(
            new CustomEvent("transform-rotate-start", {
                detail: {
                    x: evt.clientX,
                    y: evt.clientY,
                },
            })
        );
    }

    onRectDoubleClick(evt: MouseEvent) {
        this.dispatchEvent(
            new CustomEvent("transform-rect-dblclick", {
                detail: {
                    x: evt.clientX,
                    y: evt.clientY,
                },
            })
        );
    }

    @property({ type: Boolean, reflect: true })
    isTransforming: boolean = false;

    @property({ type: Boolean })
    hasMenu: boolean = false;

    @property()
    resizeLevel: ResizeLevel = "full";

    @property({ type: Number })
    width: number = 0;

    @property({ type: Number })
    height: number = 0;

    render() {
        const {
            width,
            height,
            isTransforming,
            hasMenu,
            menuButtonDot,
            resizeLevel,
            buttonHack,
        } = this;

        const dotPositions: Record<DotPos, [number, number]> = {
            tl: [0, 0],
            t: [width / 2, 0],
            tr: [width, 0],
            l: [0, height / 2],
            bl: [0, height],
            b: [width / 2, height],
            br: [width, height],
            r: [width, height / 2],
        };

        const renderRect = () => {
            //account for stroke
            const boxWidth = width + RECT_STROKE_SIZE * 2;
            const boxHeight = height + RECT_STROKE_SIZE * 2;

            const svgs = [
                svg`<svg width="${boxWidth}px" height="${boxHeight}px">
                        <rect id="fillRect" x="${RECT_STROKE_SIZE}px" y="${RECT_STROKE_SIZE}px" width="${width}px" height="${height}px" @mousedown=${this.onMoveStart} @dblclick=${this.onRectDoubleClick} />
                        </svg>`,
            ];

            svgs.push(
                //top
                svg`<svg width="${width}px" height="${RECT_STROKE_SIZE}px" style="left: ${-RECT_STROKE_SIZE}px; top: ${-RECT_STROKE_SIZE}px;">
                        <line class="rectLine" x1="0" y1="${
                            RECT_STROKE_SIZE / 2
                        }px" x2="${width}px" y2="${RECT_STROKE_SIZE / 2}px" />
                    </svg>
                    `
            );

            svgs.push(
                //bottom
                svg`<svg width="${width}px" height="${RECT_STROKE_SIZE}px" style="left: ${-RECT_STROKE_SIZE}px; top: ${height}px;">
                        <line class="rectLine" x1="0" y1="${
                            RECT_STROKE_SIZE / 2
                        }px" x2="${width}px" y2="${RECT_STROKE_SIZE / 2}px" />
                    </svg>
                    `
            );
            svgs.push(
                //left
                svg`<svg width="${RECT_STROKE_SIZE}px" height="${height}px" style="left: ${-RECT_STROKE_SIZE}px; top: ${-RECT_STROKE_SIZE}px;">
                        <line class="rectLine" y1="0" x1="${
                            RECT_STROKE_SIZE / 2
                        }px" y2="${width}px" x2="${RECT_STROKE_SIZE / 2}px" />
                    </svg>
                    `
            );

            svgs.push(
                //right
                svg`<svg width="${RECT_STROKE_SIZE}px" height="${height}px" style="left: ${width}px; top: ${-RECT_STROKE_SIZE}px;">
                        <line class="rectLine" y1="0" x1="${
                            RECT_STROKE_SIZE / 2
                        }px" y2="${width}px" x2="${RECT_STROKE_SIZE / 2}px" />
                    </svg>
                    `
            );
            return svgs;
        };

        const renderDots = () => {
            if (resizeLevel === "none") {
                return nothing;
            }

            const diameter = DOT_RADIUS * 2;

            const renderDot = (pos: DotPos) => {
                const [x, y] = dotPositions[pos];

                return svg`
                    <svg id="dot-${pos}" width="${diameter}px" height="${diameter}px" style="left: calc(${x}px - ${DOT_RADIUS}px); top: calc(${y}px - ${DOT_RADIUS}px);">
                        <circle class="dot ${pos}" cx="${DOT_RADIUS}px" cy="${DOT_RADIUS}px" r="${DOT_RADIUS}px" @mousedown=${(
                    evt: MouseEvent
                ) => this.onResizeStart(pos, evt)} />
                    </svg>
                `;
            };

            let dots = dotIds;
            if (resizeLevel === "keep-aspect-ratio") {
                // only include corners
                dots = dots.filter((id) => id.length === 2);
            }

            return dots.map(renderDot);
        };

        const renderRot = () => {
            const middle_x = width / 2;
            const renderRotLine = () => {
                return svg`
                <svg width="${ROT_STROKE_SIZE}rem" height="${ROT_LINE_DISTANCE}rem" style="left: calc(${middle_x}px - (${ROT_STROKE_SIZE}rem)/2); top: calc(${
                    RECT_STROKE_SIZE - DOT_RADIUS
                }px - ${ROT_LINE_DISTANCE}rem);">
                    <line id="rotLine" x1="${ROT_STROKE_SIZE / 2}rem" x2="${
                    ROT_STROKE_SIZE / 2
                }rem" y1="0" y2="${ROT_LINE_DISTANCE}rem" />
                    </svg>
                    `;
            };
            const renderRotButton = () => {
                let style = `width: ${ROT_BUTTON_SIZE}rem;`;
                style += ` height: ${ROT_BUTTON_SIZE}rem;`;
                style += `left: calc(${middle_x}px - (${ROT_BUTTON_SIZE}rem)/2);`;
                style += ` top: calc(${RECT_STROKE_SIZE}px - ${
                    ROT_LINE_DISTANCE + ROT_BUTTON_SIZE
                }rem);`;

                return html`<img-ui
                    .draggable=${false}
                    id="rotButton"
                    path="core/buttons/icon/rotate.svg"
                    style="${style}"
                    @mousedown=${this.onRotateStart}
                ></img-ui>`;
            };

            return html`${renderRotLine()} ${renderRotButton()}`;
        };

        const renderMenuButton = () => {
            const [x, y] = dotPositions[menuButtonDot];

            let style = `left: calc(${x}px - ${MENU_BUTTON_RADIUS}px);`;
            style += ` top: calc(${y}px - ${MENU_BUTTON_RADIUS}px);`;

            return !isTransforming && hasMenu && !buttonHack
                ? html`<div id="menu-btn" style="${style}">
                      <slot name="menu-btn"></slot>
                  </div>`
                : nothing;
        };

        return html`${renderRect()} ${renderDots()} ${renderRot()}
        ${renderMenuButton()}`;
    }
}
