import { LitElement, html, css, customElement, property } from "lit-element";

@customElement("home-testimonial-item")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: grid;
                    grid-template-columns: 156px 1fr;
                    grid-template-rows: auto auto 1fr;
                    column-gap: 40px;
                    row-gap: 16px;
                    color: #383838;
                }
                ::slotted([slot="image"]) {
                    align-self: center;
                    grid-row: 1 / -1;
                    height: 166px;
                    border-radius: 50%;
                    object-fit: cover;
                    overflow: hidden;
                }
                ::slotted([slot="name"]) {
                    font-size: 18px;
                    font-weight: 500;
                    margin: 0;
                }
                ::slotted([slot="bio"]) {
                    font-size: 14px;
                    font-weight: 300;
                    margin: 0;
                }
                ::slotted([slot="paragraph"]) {
                    font-size: 14px;
                    margin: 0;
                }
            `,
        ];
    }

    render() {
        return html`
            <slot name="image"></slot>
            <slot name="name"></slot>
            <slot name="bio"></slot>
            <slot name="paragraph"></slot>
        `;
    }
}
