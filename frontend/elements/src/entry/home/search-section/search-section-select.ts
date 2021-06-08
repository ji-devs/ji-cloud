import { LitElement, html, css, customElement, property } from 'lit-element';
import "@elements/core/overlays/anchored-overlay";

@customElement('home-search-section-select')
export class _ extends LitElement {
    static get styles() {
        return [css`
            anchored-overlay {
                width: 100%;
            }
            anchored-overlay::part(overlay) {
                box-shadow: 0 3px 6px 0 rgba(0, 0, 0, 0.08);
                padding: 16px 0;
                border-radius: 10px;
            }
            .anchor {
                cursor: pointer;
                display: flex;
                justify-content: space-between;
                padding: 0 22px;
            }
            .anchor img-ui {
                width: 30px;
                transition: transform .3s;
            }
            :host([open]) .anchor img-ui {
                transform: rotate(180deg);
            }
        `];
    }

    @property()
    value: string = "";

    @property({ type: Boolean, reflect: true })
    open: boolean = false;

    toggleOpen() {
        this.open = !this.open;
    }

    onClose() {
        this.open = false;
    }

    render() {
        return html`
            <anchored-overlay tabindex="0" positionY="bottom-out" positionX="left-in" ?open="${this.open}" @close="${this.onClose}">
                <div class="anchor" @click="${this.toggleOpen}" slot="anchor">
                    ${this.value}
                    <img-ui path="core/_common/chevron-down-blue.svg"></img-ui>
                </div>
                <div slot="overlay">
                    <slot></slot>
                </div>
            </anchored-overlay>
        `;
    }
}
