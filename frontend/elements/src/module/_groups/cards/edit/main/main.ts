import { LitElement, html, css, customElement } from "lit-element";

@customElement("main-cards")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: flex;
                    justify-content: center;
                    overflow-x: hidden;
                }
                section {
                    display: flex;
                    flex-wrap: wrap;
                    justify-content: start;
                    margin: 24px;
                    gap: 24px;
                }
                @media (min-width: 1920px) {
                    section {
                        margin: 32px;
                        gap: 32px;
                    }
                }
            `,
        ];
    }

    render() {
        return html`
            <section>
                <slot></slot>
            </section>
        `;
    }
}
