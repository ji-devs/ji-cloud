import { LitElement, html, css, customElement, property, query } from "lit-element";
import { nothing } from "lit-html";
import "@elements/core/hebrew-buttons/hebrew-buttons";

@customElement("input-wrapper")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: block;
                }
                hebrew-buttons {
                    display: block;
                    height: 0px;
                }
                hebrew-buttons::part(buttons-wrapper) {
                    display: flex;
                    justify-content: flex-end;
                    transform: translateY(-100%);
                }
                .wrapper {
                    border: solid 1px var(--light-blue-5);
                    border-radius: 14px;
                    padding: 8px 16px;
                    display: grid;
                    grid-template-columns: 1fr min-content;
                    column-gap: 2px;
                    font-size: 16px;
                    line-height: 1.5;
                    cursor: pointer;
                    z-index: 1;
                    background-color: #fff;
                }
                .wrapper:focus-within {
                    border-color: var(--dark-blue-3);
                    border-width: 2px;
                    /* removing one pixel to account for thicker border */
                    padding: 7px 15px;
                }
                :host([error]) .wrapper {
                    border-color: var(--red-alert);
                    background-color: var(--light-red-alert);
                }
                .label {
                    grid-column: 1;
                    grid-row: 1;
                    color: var(--main-blue);
                    font-weight: 500;
                }
                .wrapper:focus-within .label {
                    color: var(--dark-blue-3);
                }
                ::slotted([slot=icon]) {
                    grid-column: 2;
                    grid-row: 1 / span 2;
                    width: 24px;
                    display: flex;
                }
                ::slotted(:not([slot])) {
                    border: 0;
                    padding: 0;
                    font-size: inherit;
                    grid-column: 1;
                    color: var(--dark-gray-6);
                    background-color: transparent;
                    resize: none;
                    font-family: inherit;
                    width: 100%;
                    scrollbar-width: thin;
                    scrollbar-color: #e7f0fe transparent;
                }
                ::slotted(:not([slot]):focus) {
                    outline: 0;
                }
                ::slotted(:not([slot]))::placeholder {
                    color: var(--light-gray-4);
                }
                ::slotted(:not([slot]))::-webkit-scrollbar-track {
                    background-color: #fff;
                }
                ::slotted(:not([slot]))::-webkit-scrollbar {
                    width: 8px;
                }
                ::slotted(:not([slot]))::-webkit-scrollbar-thumb {
                    border-radius: 4px;
                    background-color: #e7f0fe;
                }
                .hint {
                    font-size: 14px;
                    font-weight: 500;
                    display: block;
                    margin: 0 8px;
                    color: #4a4a4a;
                }
                :host([error]) .hint {
                    color: var(--red-alert);
                }
            `,
        ];
    }

    @property()
    label: string = "";

    @property()
    hint: string = "";

    @property({ type: Boolean, reflect: true })
    error: boolean = false;

    @property({ type: Boolean })
    withHebrewButtons: boolean = false;

    @query("slot#main-slot")
    private mainSlot!: HTMLSlotElement;

    focus() {
        (this.mainSlot.assignedElements() as any)[0]?.focus?.();
    }

    render() {
        return html`
            ${ this.withHebrewButtons ? html`
                <hebrew-buttons></hebrew-buttons>
            ` : nothing }
            <label class="wrapper" @click="${this.focus}">
                ${ this.label ? html`<span class="label">${this.label}</span>` : nothing }
                <slot id="main-slot"></slot>
                <slot name="icon"></slot>
            </label>
            <span class="hint">${this.hint}</span>
        `;
    }
}
