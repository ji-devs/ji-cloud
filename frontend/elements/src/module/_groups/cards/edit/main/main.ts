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
                    display: grid;
                    grid-template-columns: repeat(auto-fit, 353px);
                    justify-content: center;
                    width: 100%;
                    margin: 24px;
                    gap: 24px;
                }
                @media (min-width: 1920px) {
                    section {
                        margin: 30px;
                        gap: 30px;
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
