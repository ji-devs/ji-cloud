import { LitElement, html, css, customElement, property, unsafeCSS } from 'lit-element';
import "@elements/core/images/ui";
import { mediaUi } from '@utils/path';

const STR_ADVANCED_SEARCH = "Advanced Search";

@customElement('home-search-section-advanced')
export class _ extends LitElement {
    static get styles() {
        return [css`
            :host {
                display: inline-grid;
                grid-template-columns: auto auto;
                align-items: flex-start;
            }
            main {
                padding: 64px;
                display: grid;
                grid-template-rows: auto 1fr auto;
                grid-gap: 40px;
                box-shadow: 0 3px 40px 0 rgba(0, 0, 0, 0.08);
                background-color: var(--green-3);
                border-radius: 20px;
                background-image: url("${unsafeCSS(mediaUi("entry/home/search-section/advanced-bg.svg"))}");
                background-repeat: no-repeat;
                background-position: 100% 100%;

                width: 1200px;
                height: 700px;
            }
            h2 {
                margin: 0;
                font-size: 40px;
                font-weight: 800;
                color: var(--dark-blue-4);
            }
            .selects {
                display: grid;
                grid-template-columns: repeat(3, 296px);
                grid-gap: 32px;
            }
            .search-button-wrapper {
                display: grid;
                place-content: center;
            }
        `];
    }

    render() {
        return html`
            <main>
                <h2>${STR_ADVANCED_SEARCH}</h2>
                <div class="selects">
                    <slot name="categories"></slot>
                    <slot name="affiliation"></slot>
                    <slot name="goal"></slot>
                </div>
                <div class="search-button-wrapper">
                    <slot name="search-button"></slot>
                </div>
            </main>
        `;
    }
}
