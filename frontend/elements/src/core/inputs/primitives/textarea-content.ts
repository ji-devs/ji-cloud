/* This is effectively like a contenteditable
 * but it uses a real textarea element
 * and thereby avoids many of the quirks
 *
 * the implementation uses local refs, so don't rely on the `input` property
 * e.g. only use `input` for setting the _initial_ value
 *
 * Starting in edit mode might be off if waiting for fonts to load
 */

import { LitElement, html, css, customElement, property } from "lit-element";
import { classMap } from "lit-html/directives/class-map";
import { nothing } from "lit-html";
import { styleMap } from "lit-html/directives/style-map";

export type CLICK_MODE = "single" | "double" | "none";

@customElement("input-textarea-content")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                .hiddenClickArea {
                    position: absolute;
                    top: 0;
                    left: 0;
                }
                textarea,
                span {
                    font-family: var(--font-family, Poppins);
                    /*font-size: var(--font-size, 16px);*/
                    color: var(--color, black);
                    display: none;
                    text-align: center;
                }

                span {
                    white-space: pre-wrap;
                    user-select: none;
                }
                textarea {
                    outline: 0;
                    border: none;
                    padding: 0;
                    resize: none;
                    overflow: hidden;
                    padding: 0 1px;
                    min-width: 2px;
                    min-height: 1em;
                    background-color: rgb(var(--theme-background-color));
                }

                textarea.visible,
                span.measure,
                span.visible {
                    display: inline-block;
                }

                span.measure {
                    position: absolute;
                    left: -10000px;
                    bottom: 10000px;
                }
            `,
        ];
    }

    @property()
    value: string = "";

    @property({ type: Boolean })
    editing: boolean = false;

    @property({ type: Boolean })
    disableFixedClickArea: boolean = false;

    @property({ type: Number })
    constrainWidth: number = 0;

    @property({ type: Number })
    constrainHeight: number = 0;

    @property()
    clickMode: CLICK_MODE = "double";

    @property()
    fontSize: string = "16px";

    toggleEditing = (value: boolean) => {
        this.editing = value;
        this.dispatchEvent(
            new CustomEvent("custom-toggle", {
                detail: { value },
            })
        );
    };
    onKey(evt: KeyboardEvent) {
        let { key } = evt;
        key = key.toLowerCase();
        if (key === "escape") {
            const input = this.shadowRoot?.getElementById(
                "input"
            ) as HTMLInputElement;
            input.value = this.value;
            this.toggleEditing(false);
            this.dispatchEvent(new Event("reset"));
        } else if (key === "enter") {
            //not for textarea...
            //this.dispatchChange();
        }
    }

    lastMeasuredWidth: number = 0;
    lastMeasuredHeight: number = 0;
    onInput() {
        const { constrainWidth, constrainHeight } = this;
        const input = this.shadowRoot?.getElementById(
            "input"
        ) as HTMLInputElement;
        this.resizeInput();
        if (constrainWidth && constrainHeight) {
            while (
                this.lastMeasuredWidth >= constrainWidth ||
                this.lastMeasuredHeight >= constrainHeight
            ) {
                const { value } = input;
                input.value = value.substring(0, value.length - 1);
                this.resizeInput();
            }
        }

        this.dispatchEvent(
            new CustomEvent("custom-input", {
                detail: { value: input.value },
            })
        );
    }

    resizeInput = () => {
        const input = this.shadowRoot?.getElementById(
            "input"
        ) as HTMLInputElement;
        const measure = this.shadowRoot?.getElementById(
            "measure"
        ) as HTMLInputElement;

        measure.textContent = input.value as string;

        const lastChar = input.value.charAt(input.value.length - 1);
        const rect = measure.getBoundingClientRect();

        let { width, height } = rect;
        if (lastChar === "\n" || lastChar === "\r") {
            const measureLine = this.shadowRoot?.getElementById(
                "measure-line"
            ) as HTMLInputElement;
            const lineRect = measureLine.getBoundingClientRect();
            height += lineRect.height;
        }

        input.style.width = `${width}px`;
        input.style.height = `${height}px`;

        this.lastMeasuredWidth = width;
        this.lastMeasuredHeight = height;
    };

    onGlobalMouseDown = (evt: MouseEvent) => {
        if (!evt.composedPath().includes(this as any)) {
            this.dispatchChange();
        }
    };

    firstUpdated(_changed: any) {
        this.resizeInput();
    }
    updated(changed: any) {
        if (typeof changed.get("editing") === "boolean") {
            const { editing } = this;
            this.removeGlobalListener();
            if (editing) {
                window.addEventListener("mousedown", this.onGlobalMouseDown);
                const input = this.shadowRoot?.getElementById(
                    "input"
                ) as HTMLInputElement;
                if (input) {
                    input.focus();
                    input.value = this.value;
                    input.setSelectionRange(-1, -1);
                    this.resizeInput();
                }
            }
        }
    }

    disconnectedCallback() {
        super.disconnectedCallback();
        this.removeGlobalListener();
    }

    removeGlobalListener() {
        window.removeEventListener("mousedown", this.onGlobalMouseDown);
    }

    dispatchChange = () => {
        const input = this.shadowRoot?.getElementById(
            "input"
        ) as HTMLInputElement;
        const value = input.value;
        this.dispatchEvent(
            new CustomEvent("custom-change", {
                detail: { value },
            })
        );

        this.toggleEditing(false);
    };

    render() {
        const {
            value,
            editing,
            clickMode,
            constrainWidth,
            constrainHeight,
            disableFixedClickArea,
        } = this;

        const style = styleMap({
            fontSize: this.fontSize,
        });

        const hiddenClickAreaStyle = styleMap({
            display:
                !editing && constrainWidth && constrainHeight
                    ? "block"
                    : "none",
            width: `${constrainWidth}px`,
            height: `${constrainHeight}px`,
        });

        return html`
            ${disableFixedClickArea
                ? nothing
                : html`
                      <div
                          class="hiddenClickArea"
                          style=${hiddenClickAreaStyle}
                          @dblclick=${() => {
                              if (clickMode === "double") {
                                  this.toggleEditing(true);
                              }
                          }}
                          @click=${() => {
                              if (clickMode === "single") {
                                  this.toggleEditing(true);
                              }
                          }}
                      ></div>
                  `}
            <textarea
                style=${style}
                class="${classMap({
                    visible: editing,
                })}"
                id="input"
                @input="${this.onInput}"
                @keyup="${this.onKey}"
                .value="${value}"
            ></textarea>
            <span
                style=${style}
                id="show"
                class="${classMap({
                    visible: !editing,
                })}"
                @dblclick=${() => {
                    if (clickMode === "double") {
                        this.toggleEditing(true);
                    }
                }}
                @click=${() => {
                    if (clickMode === "single") {
                        this.toggleEditing(true);
                    }
                }}
                >${value}</span
            >
            <span style=${style} id="measure" class="measure">${value}</span>
            <span style=${style} id="measure-line" class="measure">&nbsp;</span>
        `;
    }
}
