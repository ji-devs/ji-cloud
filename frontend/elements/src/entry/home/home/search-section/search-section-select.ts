import { LitElement, html, css, customElement, property } from 'lit-element';
import "@elements/core/overlays/anchored-overlay";

@customElement('home-search-section-select')
export class _ extends LitElement {
    static get styles() {
        return [css`
            .anchor {
                cursor: pointer;
                display: flex;
                justify-content: space-between;
                padding: 0 10px;
            }
            .value {
                color: var(--dark-gray-6);
                overflow: hidden;
                text-overflow: ellipsis;
                white-space: nowrap;
            }
            .anchor img-ui {
                width: 30px;
                transition: transform .3s;
            }
            :host([open]) .anchor img-ui {
                transform: rotate(180deg);
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
                padding-bottom: 14px;
            }
        `];
    }

    @property()
    value: string = "";

    @property({ type: Boolean, reflect: true })
    open: boolean = false;

    @property({ type: Boolean })
    multiple: boolean = false;

    private onBaseOpenChange(e: CustomEvent) {
        this.open = e.detail.open;
    }

    render() {
        return html`
            <style>
                base-select::part(anchored-overlay_overlay) {
                    width: ${this.clientWidth}px;
                }
            </style>

            <base-select
                ?multiple=${this.multiple}
                ?open=${this.open}
                @custom-open-change="${this.onBaseOpenChange}"
            >
                <div slot="anchor" class="anchor">
                    <div class="value">${this.value}</div>
                    <img-ui slot="icon" class="icon" path="core/_common/chevron-down-blue.svg"></img-ui>
                </div>
                <slot></slot>
            </base-select>
        `;
    }
}
