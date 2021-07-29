import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/hebrew-buttons/hebrew-buttons";
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
                    margin: 40px 0;
                    height: 1px;
                }
                .controls {
                    display: grid;
                    row-gap: 22px;
                }
                hebrew-buttons {
                    place-self: end;
                    margin-bottom: 18px;
                }
                :host([controlsDisabled]) .controls {
                    filter: opacity(.5);
                    pointer-events: none;
                }
                .button-collection {
                    display: flex;
                    border: solid 1px var(--light-blue-5);
                    border-radius: 14px;
                    justify-content: space-evenly;
                    padding: 14px 0;
                }
                .row {
                    display: grid;
                    column-gap: 22px;
                    justify-content: space-between;
                }
                .first {
                    grid-template-columns: 1fr 1fr;
                }
                .second {
                    grid-template-columns: 124px 124px 204px;
                }
                .third {
                    grid-template-columns: 1fr 1fr;
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
        }
    }

    render() {
        return html`
            <div class="insert">
                <slot name="insert-button"></slot>
            </div>
            <div class="divider"></div>
            <hebrew-buttons full .positionKeyboard="${this.positionHebrewKeyboard}"></hebrew-buttons>
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
                    <div class="button-collection">
                        <slot name="bold"></slot>
                        <slot name="italic"></slot>
                        <slot name="underline"></slot>
                    </div>
                    <div class="button-collection">
                        <slot name="colors"></slot>
                    </div>
                    <div class="button-collection">
                        <slot name="align-left"></slot>
                        <slot name="align-center"></slot>
                        <slot name="align-right"></slot>
                        <slot name="indent"></slot>
                        <slot name="outdent"></slot>
                    </div>
                </div>
                <div class="row third">
                    <slot name="font"></slot>
                    <slot name="weight"></slot>
                </div>
            </div>
        `;
    }
}
