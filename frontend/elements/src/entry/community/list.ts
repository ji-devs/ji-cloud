import { LitElement, html, css, customElement, property } from "lit-element";

@customElement("community-list")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: grid;
                    align-items: center;
                    gap: 24px;
                }
                @media (min-width: 1024px) {
                    :host {
                        gap: 10px;
                        grid-template-columns: auto auto;
                        justify-content: space-between;
                    }
                }
                h1 {
                    color: var(--dark-blue-4);
                    font-size: 25px;
                    line-height: 1em;
                    font-weight: 800;
                    margin: 0;
                    text-align: center;
                    grid-column: 1 / -1;
                }
                @media (min-width: 1024px) {
                    h1 {
                        grid-column: 1;
                        text-align: left;
                    }
                }
                ::slotted([slot=create-button]) {
                    justify-self: center;
                }
                @media (min-width: 1024px) {
                    ::slotted([slot=create-button]) {
                        justify-self: end;
                    }
                }
                ::slotted([slot=pagination]),
                .items {
                    grid-column: 1 / -1;
                }
                .items {
                    display: grid;
                    row-gap: 20px;
                    column-gap: 20px;
                    align-items: center;
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
            <slot name="popup"></slot>
        `;
    }
}
