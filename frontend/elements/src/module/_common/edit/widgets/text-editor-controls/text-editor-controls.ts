import { LitElement, html, css, customElement, property } from "lit-element";
import { KEYBOARD_HEIGHT } from "@elements/core/hebrew-buttons/hebrew-buttons";

@customElement("text-editor-controls")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: grid;
                }
                .divider {
                    background-color: var(--light-blue-4);
                    margin: 20px 0;
                    height: 1px;
                }
                .controls {
                    display: grid;
                    row-gap: 10px;
                }
                ::slotted(hebrew-buttons) {
                    place-self: end;
                    margin-bottom: 14px;
                }
                :host([controlsDisabled]) .controls {
                    filter: opacity(0.5);
                    pointer-events: none;
                }
                .button-collection {
                    display: flex;
                    border: solid 1px var(--light-blue-5);
                    border-radius: 12px;
                    justify-content: space-evenly;
                    padding: 4px 0;
                }
                .row {
                    display: grid;
                    justify-content: space-between;
                    column-gap: 10px;
                    row-gap: 10px;
                }
                .first {
                    grid-template-columns: 1fr min-content;
                }
                .second, .third {
                    justify-content: stretch;
                }
                .fourth {
                    grid-template-columns: 1fr 1fr;
                }
                .fourth .alignment {
                    grid-column: 1 / -1;
                }
                ::slotted(input-select) {
                    /* use background color of sidebar */
                    --background-color: #e9eff8;
                }
                ::slotted(anchored-overlay[slot="colors"]) {
                    display: block;
                }
                .button-collection.color {
                    display: block;
                }
            `,
        ];
    }

    @property({ type: Boolean, reflect: true })
    controlsDisabled: boolean = false;

    connectedCallback() {
        super.connectedCallback();
        this.setAttribute("tabindex", "0");
    }

    private positionHebrewKeyboard(rect: DOMRect) {
        return {
            y: rect.top - KEYBOARD_HEIGHT,
            x: 0,
        };
    }

    render() {
        return html`
            <div class="insert">
                <slot name="insert-button"></slot>
            </div>
            <div class="divider"></div>
            <slot name="hebrew-buttons"></slot>
            <div class="controls">
                <div class="row first">
                    <div class="button-collection">
                        <slot name="h1"></slot>
                        <slot name="h2"></slot>
                        <slot name="p1"></slot>
                        <slot name="p2"></slot>
                    </div>
                    <slot name="font-size"></slot>
                </div>
                <div class="row second">
                    <slot name="font"></slot>
                </div>
                <div class="row third">
                    <slot name="weight"></slot>
                </div>
                <div class="row fourth">
                    <div class="button-collection">
                        <slot name="bold"></slot>
                        <slot name="italic"></slot>
                        <slot name="underline"></slot>
                    </div>
                    <div class="button-collection color">
                        <slot name="colors"></slot>
                    </div>
                    <div class="button-collection alignment">
                        <slot name="align-left"></slot>
                        <slot name="align-center"></slot>
                        <slot name="align-right"></slot>
                        <slot name="left-to-right"></slot>
                        <slot name="right-to-left"></slot>
                    </div>
                </div>
            </div>
        `;
    }
}
