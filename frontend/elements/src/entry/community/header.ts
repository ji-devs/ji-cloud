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
            `,
        ];
    }

    render() {
        return html`
            <h2>${STR_COMMUNITY}</h2>
            <slot name="nav"></slot>
            <div>
                <slot name="search-input"></slot>
                <slot name="search-button"></slot>
            </div>
        `;
    }
}
