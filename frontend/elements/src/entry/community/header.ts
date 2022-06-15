import { LitElement, html, css, customElement, property } from "lit-element";

const STR_COMMUNITY = "Community";

@customElement("community-header")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: grid;
                    grid-template-columns: auto auto;
                    justify-content: space-between;
                    padding: 40px 30px;
                    background-color: #f2777f;
                }
                h2 {
                    margin: 0;
                    color: #fed758;
                    font-size: 48px;
                    font-weight: 900;
                }
                ::slotted([slot=nav]) {
                    display: flex;
                    column-gap: 40px;
                }
                .search-section {
                    display: grid;
                    grid-template-columns: auto auto;
                    gap: 16px;
                }
                ::slotted([slot=search-button]) {
                    width: 48px;
                    height: 48px;
                    background-color: var(--main-yellow);
                    color: #f2777f;
                    display: inline-grid;
                    place-content: center;
                    border-radius: 50%;
                    font-size: 24px;
                    transition: box-shadow .1s;
                    box-shadow: 0 3px 6px 0 rgba(0, 0, 0, 0.16);
                }
                ::slotted([slot=search-button]:active) {
                    box-shadow: 0 0px 2px 0 rgba(0, 0, 0, 0.16);
                }
                ::slotted([slot=search-input]) {
                    width: 420px;
                    height: 48px;
                    padding: 0 20px;
                    border-radius: 24px;
                    background-color: #fee0e2;
                    border: 0;
                    font-size: 18px;
                    color: var(--dark-blue-8);
                }
            `,
        ];
    }

    render() {
        return html`
            <h2>${STR_COMMUNITY}</h2>
            <slot name="nav"></slot>
            <div class="search-section">
                <slot name="search-input"></slot>
                <slot name="search-button"></slot>
            </div>
        `;
    }
}
