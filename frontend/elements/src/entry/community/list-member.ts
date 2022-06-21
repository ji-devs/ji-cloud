import { LitElement, html, css, customElement, property } from "lit-element";

@customElement("community-list-member")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                p {
                    margin: 0;
                }
                :host {
                    cursor: pointer;
                    min-height: 88px;
                    padding: 12px 24px;
                    display: grid;
                    align-items: center;
                    justify-content: space-between;
                    column-gap: 24px;
                    border-radius: 16px;
                    box-shadow: 0 0 6px 0 rgba(0, 0, 0, 0.08);
                    border: solid 1px var(--light-orange-3);
                    background-color: #ffffff;
                    font-size: 16px;
                    font-weight: 500;
                    color: var(--dark-gray-6);
                }
                ::slotted([slot=img]) {
                    height: 64px;
                    width: 64px;
                    border-radius: 50%;
                }
                .location {
                    display: inline-grid;
                }
            `,
        ];
    }

    @property()
    name: string = "";

    @property()
    city: string = "";

    @property()
    state: string = "";

    @property()
    language: string = "";

    render() {
        return html`
            <slot name="img"></slot>
            <p class="name">${this.name}</p>
            <p class="location">
                <span>${this.city}</span>
                <span>${this.state}</span>
            </p>
            <p class="language">${this.language}</p>
            <slot name="status"></slot>
        `;
    }
}
