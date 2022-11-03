/* This is effectively like a contenteditable
 * but it uses a real textarea element
 * and thereby avoids many of the quirks
 *
 * the implementation uses local refs, so don't rely on the `input` property
 * e.g. only use `input` for setting the _initial_ value
 *
 * Starting in edit mode might be off if waiting for fonts to load
 */

import { LitElement, html, css, customElement, property, query, PropertyValues } from "lit-element";
import { styleMap } from "lit-html/directives/style-map";
import { closestPierceShadow } from "@utils/dom";

export type CLICK_MODE = "single" | "double" | "none";

@customElement("input-textarea-content")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                textarea,
                .measure {
                    font-family: var(--font-family, Poppins);
                    color: var(--color, black);
                    text-align: center;
                }
                textarea {
                    outline: 0;
                    border: none;
                    padding: 0;
                    resize: none;
                    overflow: hidden;
                    min-width: 2px;
                    min-height: 1em;
                    background-color: rgb(var(--theme-background-color));
                }
                .measure {
                    white-space: pre-wrap;
                    user-select: none;
                    display: inline-block;
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

    toggleEditing = (value: boolean) => {
        this.editing = value;
        this.dispatchEvent(
            new CustomEvent("custom-toggle", {
                detail: { value },
            })
        );
    };

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
        this.textarea.style.height = `0px`;
        this.textarea.style.height = `${this.textarea.scrollHeight}px`;

        const lastChar = this.textarea.value.charAt(this.textarea.value.length - 1);
        const rect = this.textarea.getBoundingClientRect();

        let { width, height } = rect;
        if (lastChar === "\n" || lastChar === "\r") {
            const measureLine = this.shadowRoot?.getElementById(
                "measure-line"
            ) as HTMLInputElement;
            const lineRect = measureLine.getBoundingClientRect();
            height += lineRect.height;
        }

        this.lastMeasuredWidth = width;
        this.lastMeasuredHeight = height;
    };

    onBlur(e: FocusEvent) {
        let relatedTarget = closestPierceShadow(e.relatedTarget as Node, "hebrew-keyboard, hebrew-buttons");

        // if is keyboard, keyboard with refocus. If is hebrew-buttons, refocus here
        if(relatedTarget?.matches("hebrew-buttons")) {
            this.textarea?.focus();
        } else {
            this.dispatchChange();
            this.toggleEditing(false);
        }
    }

    onFocus() {
        this.toggleEditing(true);
    }

    firstUpdated() {
        this.resizeInput();
    }
    updated(changed: PropertyValues) {
        if (changed.has("editing")) {
            const { editing } = this;
            if (editing) {
                if (this.textarea) {
                    this.textarea.focus();
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
    };

    render() {
        const style = styleMap({
            fontSize: this.fontSize,
        });

        return html`
            <textarea
                style=${style}
                @input=${this.onInput}
                @blur=${this.onBlur}
                @focus=${this.onFocus}
                .value=${this.value}
            ></textarea>
            <span style=${style} id="measure-line" class="measure">&nbsp;</span>
        `;
    }
}
