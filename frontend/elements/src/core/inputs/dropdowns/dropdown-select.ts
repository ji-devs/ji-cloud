import { LitElement, html, css, customElement, property } from "lit-element";
import { nothing } from "lit-html";
import "@elements/core/overlays/anchored-overlay";

@customElement("dropdown-select")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                anchored-overlay {
                    display: block;
                }
                .input {
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
                :host([open]) .input {
                    border-color: var(--dark-blue-3);
                    border-width: 2px;
                    /* removing one pixel to account for thicker border */
                    padding: 7px 15px;
                    position: relative;
                    z-index: 2;
                }
                :host([error]) .input {
                    border-color: var(--red-alert);
                    background-color: var(--light-red-alert);
                }
                .input .label {
                    grid-column: 1;
                    grid-row: 1;
                    color: var(--main-blue);
                    font-weight: 500;
                }
                :host([open]) .label {
                    color: var(--dark-blue-3);
                }
                .input .value, .input .placeholder {
                    grid-column: 1;
                    overflow: hidden;
                    text-overflow: ellipsis;
                    white-space: nowrap;
                }
                .input .value {
                    color: var(--dark-gray-6);
                }
                .input .placeholder {
                    color: var(--light-gray-4);
                }
                .input .icon {
                    grid-column: 2;
                    grid-row: 1 / span 2;
                    width: 24px;
                    transition: transform .3s;
                    display: flex;
                }
                :host([open]) .icon {
                    transform: rotate(180deg);
                }
                anchored-overlay::part(overlay) {
                    width: 100%;
                    padding-top: 30px;
                    margin-top: -30px;
                    border-radius: 14px;
                    box-shadow: 0 3px 6px 0 rgba(0, 0, 0, 0.16);
                    background-color: #fff;
                    z-index: 1;
                }
                :host(:not([nested])) anchored-overlay::part(overlay) {
                    max-height: 250px;
                    overflow: auto;
                }
            `,
        ];
    }

    @property()
    label?: string;

    @property()
    value: string = "";

    @property()
    placeholder: string = "";

    @property({type: Boolean, reflect: true})
    error: boolean = false;

    @property({ type: Boolean, reflect: true })
    open: boolean = false;

    // when nested is enabled overflow is disabled since the nested items are overflowing the container, there might be ways to get around this issue
    @property({ type: Boolean, reflect: true })
    nested: boolean = false;

    public toggleOpen() {
        this.open = !this.open;
    }

    render() {
        return html`
            <anchored-overlay
                ?open="${this.open}"
                @close="${() => this.open = false}"
                ?autoClose="${false}"
                positionY="bottom-out"
                positionX="left-in"
            >
                <div slot="anchor" class="input" @click=${() => this.toggleOpen()}>
                    ${ this.label ? html`<span class="label">${this.label}</span>` : nothing }
                    ${ this.value ? (
                        html`<span class="value" title="${this.value}">${this.value}</span>`
                    ) : (
                        html`<span class="placeholder">${this.placeholder}</span>`
                    )}
                    <img-ui class="icon" path="core/_common/chevron-down-blue.svg"></img-ui>
                </div>
                <slot slot="overlay"></slot>
            </anchored-overlay>
        `;
    }
}
