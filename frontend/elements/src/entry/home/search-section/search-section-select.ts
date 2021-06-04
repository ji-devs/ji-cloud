import { LitElement, html, css, customElement, property, query } from 'lit-element';
import "@elements/core/overlays/anchored-overlay";
import { _ as AnchoredOverlay } from "@elements/core/overlays/anchored-overlay";

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
                border-radius: 12px;
            }
            .anchor {
                min-height: 1em;
            }
        `];
    }

    @property()
    value: string = "";

    @query("anchored-overlay")
    anchoredOverlay!: AnchoredOverlay;

    toggleOpen() {
        this.anchoredOverlay.open = !this.anchoredOverlay.open;
    }

    render() {
        return html`
            <anchored-overlay tabindex="0" positionY="bottom-out" positionX="left-in">
                <div class="anchor" @click="${this.toggleOpen}" slot="anchor">${this.value}</div>
                <div slot="overlay">
                    <slot></slot>
                </div>
            </anchored-overlay>
        `;
    }
}
