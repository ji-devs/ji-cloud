import {
    LitElement,
    html,
    css,
    customElement,
    property,
    unsafeCSS,
    query,
} from "lit-element";
import "@elements/core/overlays/anchored-overlay";
import { AnchoredOverlay } from "@elements/core/overlays/anchored-overlay";
import "@elements/core/images/ui";
import { mediaUi } from "@utils/path";

const STR_ADVANCED_SEARCH = "Advanced Search";

@customElement("home-search-section-advanced")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                anchored-overlay::part(overlay) {
                    background-color: transparent;
                }
                main {
                    text-align: left;
                    padding: 64px;
                    display: grid;
                    grid-template-rows: auto 1fr auto;
                    grid-gap: 40px;
                    box-shadow: 0 3px 40px 0 rgba(0, 0, 0, 0.08);
                    border-radius: 20px;
                    background-color: var(--green-3);
                    background-image: url("${unsafeCSS(
                        mediaUi("entry/home/search-section/advanced-bg.svg")
                    )}");
                    background-repeat: no-repeat;
                    background-position: 100% 100%;
                    background-size: 400px;
                    width: 1000px;
                    height: 400px;
                    box-sizing: border-box;
                }
                h2 {
                    margin: 0;
                    font-size: 40px;
                    font-weight: 800;
                    color: var(--dark-blue-4);
                }
                .selects {
                    display: grid;
                    grid-template-columns: repeat(3, 250px);
                    grid-gap: 32px;
                    align-items: start;
                }
                ::slotted(input-select) {
                    --background-color: #ffffff;
                }
                .search-button-wrapper {
                    display: grid;
                    place-content: center;
                }
            `,
        ];
    }

    @query("anchored-overlay")
    anchoredOverlay!: AnchoredOverlay;

    private toggleOpen() {
        this.anchoredOverlay.open = !this.anchoredOverlay.open;
    }

    render() {
        return html`
            <anchored-overlay positionY="bottom-out" positionX="right-in">
                <slot
                    slot="anchor"
                    name="opener"
                    @click="${this.toggleOpen}"
                ></slot>
                <main slot="overlay">
                    <h2>${STR_ADVANCED_SEARCH}</h2>
                    <div class="selects">
                        <slot name="categories"></slot>
                        <slot name="affiliation"></slot>
                    </div>
                    <div class="search-button-wrapper">
                        <slot name="search-button"></slot>
                    </div>
                </main>
            </anchored-overlay>
        `;
    }
}
