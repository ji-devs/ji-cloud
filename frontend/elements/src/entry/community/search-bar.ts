import { LitElement, html, css, customElement, property } from "lit-element";

@customElement("community-search-bar")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: grid;
                    grid-template-columns: auto auto;
                    gap: 12px;
                    width: 100%;
                }
                ::slotted([slot=search-button]) {
                    width: 36px;
                    height: 36px;
                    background-color: var(--main-yellow);
                    color: #f2777f;
                    display: inline-grid;
                    place-content: center;
                    border-radius: 50%;
                    font-size: 20px;
                    transition: box-shadow .1s;
                    box-shadow: 0 3px 6px 0 rgba(0, 0, 0, 0.16);
                }
                ::slotted([slot=search-button]:active) {
                    box-shadow: 0 0px 2px 0 rgba(0, 0, 0, 0.16);
                }
                ::slotted([slot=search-input]) {
                    width: 100%;
                    height: 36px;
                    padding: 0 16px;
                    border-radius: 20px;
                    background-color: #fee0e2;
                    border: 0;
                    font-size: 16px;
                    color: var(--dark-blue-8);
                }
            `,
        ];
    }

    render() {
        return html`
            <slot name="search-input"></slot>
            <slot name="search-button"></slot>
        `;
    }
}
