import { LitElement, html, css, customElement, property } from 'lit-element';
import '@elements/core/overlays/anchored-overlay';

const STR_FILTER = "Filter";
const STR_SEARCH_IN = "Search in";
const STR_IMAGE_STYLE = "Image style";

@customElement('image-search-filters')
export class _ extends LitElement {

    static get styles() {
        return [css`
            button {
                font-family: Poppins;
                font-weight: 500;
                font-stretch: normal;
                font-style: normal;
                border: 0;
                padding: 0;
                background-color: transparent;
                cursor: pointer;
                display: flex;
                align-items: center;
                column-gap: 8px;
                font-size: 14px;
            }
            @media (min-width: 1920px) {
                button {
                    font-size: 16px;
                }
            }
            .caret {
                transition: transform .2s;
            }
            :host([open]) .caret {
                transform: rotate(180deg);
            }
            .overlay {
                width: 235px;
            }
            .source-options {
                display: flex;
                column-gap: 24px;
            }
            section {
                padding: 16px;
                display: grid;
                row-gap: 10px;
            }
            .style-section {
                padding: 16px 2px;
            }
            .style-section h4 {
                padding: 0 14px;
            }
            section:not(:last-child) {
                border-bottom: solid 1px var(--light-gray-1);
            }
            h4 {
                font-size: 16px;
                font-weight: 500;
                color: var(--main-blue);
                margin: 0;
            }
        `];
    }

    @property({type: Boolean, reflect: true})
    open: boolean = false;

    private openClick() {
        this.open = !this.open;
    }

    render() {
        return html`
            <anchored-overlay
                .open=${this.open}
                .autoClose=${false}
                @close=${() => this.open = false}
                .styled=${true}
                positionX="right-in"
            >
                <button slot="anchor" @click="${this.openClick}">
                    ${STR_FILTER}
                    <img-ui class="caret" path="module/_common/edit/widgets/sidebar/image-select/open-filters-icon.svg"></img-ui>
                </button>
                <div slot="overlay" class="overlay">
                    <section class="source-section">
                        <h4>${STR_SEARCH_IN}</h4>
                        <div class="source-options">
                            <slot name="source-options"></slot>
                        </div>
                    </section>
                    <section class="style-section">
                        <h4>${STR_IMAGE_STYLE}</h4>
                        <div class="style-options">
                            <slot name="style-options"></slot>
                        </div>
                    </section>
                </div>
            </anchored-overlay>
        `;
    }
}
