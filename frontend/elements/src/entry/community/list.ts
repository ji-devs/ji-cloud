import { LitElement, html, css, customElement, property } from "lit-element";

@customElement("community-list")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: grid;
                    grid-template-columns: auto auto;
                    justify-content: space-between;
                    padding: 40px 30px;
                    background-color: #fff6d9;
                }
                ::slotted([slot=pagination]),
                .items {
                    grid-column: 1 / span 2;
                }
                .items {
                    display: grid;
                    row-gap: 24px;
                }
            `,
        ];
    }

    @property()
    header: string = "";

    render() {
        return html`
            <h1>${this.header}</h1>
            <slot name="create-button"></slot>
            <slot name="pagination"></slot>
            <div class="items">
                <slot name="sort-header"></slot>
                <slot name="items"></slot>
            </div>
        `;
    }
}
