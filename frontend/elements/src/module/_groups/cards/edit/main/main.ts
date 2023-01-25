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
                    grid-template-columns: repeat(auto-fill, 242px);
                    justify-content: center;
                    width: 100%;
                    margin: 48px 16px;
                    column-gap: 28px;
                    row-gap: 38px;
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
