/* This is effectively like a contenteditable
 * but it uses a real textarea element
 * and thereby avoids many of the quirks
 *
 * the implementation uses local refs, so don't rely on the `input` property
 * e.g. only use `input` for setting the _initial_ value
 *
 * Starting in edit mode might be off if waiting for fonts to load
 */

import { LitElement, html, css, customElement, property, query } from "lit-element";
import { classMap } from "lit-html/directives/class-map";
import { nothing } from "lit-html";
import { styleMap } from "lit-html/directives/style-map";
import { closestPierceShadow } from "@utils/dom";

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
                    /* display: none; */
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

    @query('textarea')
    textarea!: HTMLTextAreaElement;

    @query('#measure')
    measure!: HTMLElement;

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
            this.textarea.value = this.value;
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
        this.resizeInput();
        if (constrainWidth && constrainHeight) {
            while (
                this.lastMeasuredWidth >= constrainWidth ||
                this.lastMeasuredHeight >= constrainHeight
            ) {
                const { value } = this.textarea;
                this.textarea.value = value.substring(0, value.length - 1);
                this.resizeInput();
            }
        }

        this.dispatchEvent(
            new CustomEvent("custom-input", {
                detail: { value: this.textarea.value },
            })
        );
    }

    resizeInput = () => {
        this.measure.textContent = this.textarea.value as string;

        const lastChar = this.textarea.value.charAt(this.textarea.value.length - 1);
        const rect = this.measure.getBoundingClientRect();

        let { width, height } = rect;
        if (lastChar === "\n" || lastChar === "\r") {
            const measureLine = this.shadowRoot?.getElementById(
                "measure-line"
            ) as HTMLInputElement;
            const lineRect = measureLine.getBoundingClientRect();
            height += lineRect.height;
        }

        this.textarea.style.width = `${width}px`;
        this.textarea.style.height = `${height}px`;

        this.lastMeasuredWidth = width;
        this.lastMeasuredHeight = height;
    };

    onBlur(e: FocusEvent) {
        let relatedTarget = closestPierceShadow(e.relatedTarget as Node, "hebrew-keyboard, hebrew-buttons");

        // if is keyboard, keyboard with refocus. If is hebrew-buttons, refocus here
        if(relatedTarget?.matches("hebrew-buttons")) {
            this.shadowRoot?.querySelector("textarea")?.focus();
        }

        if (!relatedTarget) {
            this.dispatchChange();
        }
    }

    firstUpdated(_changed: any) {
        this.resizeInput();
    }
    updated(changed: any) {
        if (typeof changed.get("editing") === "boolean") {
            const { editing } = this;
            if (editing) {
                if (this.textarea) {
                    this.textarea.focus();
                    this.textarea.value = this.value;
                    this.textarea.setSelectionRange(-1, -1);
                    this.resizeInput();
                }
            }
        }
    }

    disconnectedCallback() {
        super.disconnectedCallback();
    }

    dispatchChange = () => {
        const value = this.textarea.value;
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
                .readOnly=${!editing}
                @input="${this.onInput}"
                @keyup="${this.onKey}"
                @blur="${this.onBlur}"
                @focus=${() => this.editing = true}
                .value="${value}"
            ></textarea>
            <!-- <span
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
            > -->
            <span style=${style} id="measure" class="measure">${value}</span>
            <span style=${style} id="measure-line" class="measure">&nbsp;</span>
        `;
    }
}
