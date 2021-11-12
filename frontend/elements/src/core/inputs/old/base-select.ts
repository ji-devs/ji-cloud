import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/overlays/anchored-overlay";

// this can also be used as a base class
@customElement("input-base-select")
export class BaseSelect extends LitElement {
    static get styles() {
        return [
            css`
                anchored-overlay {
                    display: block;
                }
                .input {
                    display: grid;
                }
                .input .value,
                .input .placeholder {
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
                :host(:not([nested])) anchored-overlay::part(overlay) {
                    max-height: 250px;
                    overflow: auto;
                }
            `,
        ];
    }

    @property()
    value: string = "";

    @property()
    placeholder: string = "";

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
                @close="${() => (this.open = false)}"
                ?autoClose="${false}"
                positionY="bottom-out"
                positionX="left-in"
                tabindex="0"
                part="anchored-overlay"
            >
                <div
                    slot="anchor"
                    class="input"
                    @click=${() => this.toggleOpen()}
                >
                    ${this.value
                        ? html`<span class="value" title="${this.value}"
                              >${this.value}</span
                          >`
                        : html`<span class="placeholder"
                              >${this.placeholder}</span
                          >`}
                </div>
                <slot slot="overlay"></slot>
            </anchored-overlay>
        `;
    }
}
