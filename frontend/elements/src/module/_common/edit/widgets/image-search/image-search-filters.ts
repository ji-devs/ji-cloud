import { LitElement, html, css, customElement, property } from 'lit-element';

const STR_FILTER = "Filter";
const STR_SEARCH_IN = "Search in";
const STR_IMAGE_STYLE = "Image style";

@customElement('image-search-filters')
export class _ extends LitElement {

    static get styles() {
        return [css`
            :host {
                position: relative;
                display: flex;
                flex-direction: column;
                align-items: flex-end;
            }
            button {
                font-family: Poppins;
                font-size: 16px;
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
            }
            .caret {
                transition: transform .2s;
            }
            :host([open]) .caret {
                transform: rotate(180deg);
            }
            .overlay {
                position: absolute;
                box-shadow: rgb(0 0 0 / 20%) 0px 0px 5px;
                background-color: #fff;
                width: 235px;
                right: 0;
                top: 25px;
                border-radius: 14px;
                display: none;
            }
            :host([open]) .overlay {
                display: block;
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
            <button @click="${this.openClick}">
                ${STR_FILTER}
                <img-ui class="caret" path="module/_common/edit/widgets/sidebar/image-select/open-filters-icon.svg"></img-ui>
            </button>
            <div class="overlay">
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
        `;
    }
}
