import {
    LitElement,
    svg,
    html,
    css,
    customElement,
    property,
} from "lit-element";

//If changing this, also update the CSS for .rectLine
const RECT_STROKE_SIZE = 3;

@customElement("highlight-box")
export class TransformBox extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    touch-events: none;
                    pointer-events: none;
                }
                :host(:focus) {
                    outline: none;
                }
                svg {
                    touch-events: none;
                    pointer-events: none;
                    position: absolute;
                    top: 0;
                    left: 0;
                }

                :host {
                    position: absolute;
                    top: 0;
                    left: 0;
                    z-index: 1;
                }

                .rectLine {
                    stroke: var(--orange-1);
                    stroke-width: 3;
                    stroke-dasharray: 4px;
                }
                #fillRect {
                    fill-opacity: 0;
                }
            `,
        ];
    }

    @property({ type: Number })
    width: number = 0;

    @property({ type: Number })
    height: number = 0;

    render() {
        const {
            width,
            height,
        } = this;

        const renderRect = () => {
            //account for stroke
            const boxWidth = width + RECT_STROKE_SIZE * 2;
            const boxHeight = height + RECT_STROKE_SIZE * 2;

            const svgs = [
                svg`<svg width="${boxWidth}px" height="${boxHeight}px">
                        <rect id="fillRect" x="${RECT_STROKE_SIZE}px" y="${RECT_STROKE_SIZE}px" width="${width}px" height="${height}px" />
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

        return html`${renderRect()}`
    }
}
