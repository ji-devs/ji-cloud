import { LitElement, html, css, customElement, property } from "lit-element";

@customElement("home-new-item")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    padding: 20px 0;
                }

                .inside-wrapper {
                    margin: 0 24px;
                    height: 540px;
                    background-color: #ffffff;
                    border-radius: 25px;
                    box-shadow: 0 3px 20px 0 rgba(0, 0, 0, 0.16);
                    overflow: hidden;
                    display: grid;
                    grid-template-columns: 60% 40%;
                }

                ::slotted([slot="image"]) {
                    height: 540px;
                    width: 100%;
                    object-fit: cover;
                }

                .content {
                    display: grid;
                    grid-template-rows: auto 1fr auto;
                    row-gap: 32px;
                    padding: 56px;
                }

                ::slotted(*) {
                    margin: 0;
                }
                ::slotted([slot="subtitle"]) {
                    font-size: 40px;
                    font-weight: 300;
                    color: #f2777f;
                }
                ::slotted([slot="lines"]) {
                    font-size: 20px;
                    color: var(--dark-gray-6);
                }
            `,
        ];
    }

    render() {
        return html`
            <div class="inside-wrapper">
                <slot name="image"></slot>
                <div class="content">
                    <slot name="subtitle"></slot>
                    <slot name="lines"></slot>
                    <slot name="button"></slot>
                </div>
            </div>
        `;
    }
}
