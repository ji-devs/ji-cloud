import { LitElement, html, css, customElement, property, query } from "lit-element";
import "@elements/core/inputs/primitives/select/base-select";
import "@elements/core/inputs/wrapper";
import "@elements/core/images/ui";
import { BaseSelect } from "@elements/core/inputs/primitives/select/base-select";


@customElement("input-select")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: block;
                }
                :host([open]) {
                    box-shadow: 0 4px 4px 0 rgba(0, 0, 0, 0.16);
                }
                base-select {
                    display: block;
                }
                base-select::part(anchored-overlay) {
                    display: block;
                }
                base-select::part(anchored-overlay_overlay) {
                    border-radius: 0 0 14px 14px;
                    box-shadow: 0 4px 4px 0 rgba(0, 0, 0, 0.16);
                    background-color: white;
                    padding-bottom: 8px;
                }
                .label-placeholder {
                    overflow: hidden;
                    text-overflow: ellipsis;
                    white-space: nowrap;
                }
                .value {
                    color: var(--dark-gray-6);
                }
                .placeholder {
                    color: var(--light-gray-4);
                }
                .icon {
                    transition: transform .3s;
                }
                :host([open]) .icon {
                    transform: rotate(180deg);
                }
            `,
        ];
    }

    @property()
    label: string = "";

    @property({ type: Boolean })
    error: boolean = false;

    @property()
    hint: string = "";

    @property()
    value: string = "";

    @property()
    placeholder: string = "";

    @property({ type: Boolean, reflect: true })
    open: boolean = false;

    @property({ type: Boolean })
    multiple: boolean = false;

    @query("base-select")
    baseSelect!: BaseSelect;

    private onBaseOpenChange(e: CustomEvent) {
        this.open = e.detail.open;
    }

    createRenderRoot() {
        return this.attachShadow({ mode: 'open', delegatesFocus: true });
    }

    render() {
        return html`
            <style>
                base-select::part(anchored-overlay_overlay) {
                    width: ${this.clientWidth}px;
                }
            </style>

            <base-select
                ?open="${this.open}"
                @custom-open-change="${this.onBaseOpenChange}"
                ?multiple=${this.multiple}
            >
                <input-wrapper
                    slot="anchor"
                    label="${this.label}"
                    hint="${this.hint}"
                    ?error="${this.error}"
                    ?focus-within=${this.open}
                >
                    <div class="label-placeholder">
                        ${ this.value ? (
                            html`<span class="value">${this.value}</span>`
                        ) : (
                            html`<span class="placeholder">${this.placeholder}</span>`
                        )}
                    </div>
                    <img-ui slot="icon" class="icon" path="core/_common/chevron-down-blue.svg"></img-ui>
                </input-wrapper>
                <slot></slot>
            </base-select>
        `;
    }
}
