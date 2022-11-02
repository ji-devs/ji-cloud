import { LitElement, html, css, customElement, property } from "lit-element";

@customElement("question-banner")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    position: absolute;
                    top: 40rem;
                    left: 0;
                    display: flex;
                    justify-content: center;
                    width: 100%;
                }
                section {
                    padding: 8px 24px;
                    border-radius: 32px;
                    box-shadow: 0 3px 6px 0 rgba(0, 0, 0, 0.16);
                    background-color: var(--white);
                    display: inline-block;
                    font-size: 32rem;
                    font-weight: bold;
                    text-align: center;
                    color: var(--dark-gray-6);
                }
            `,
        ];
    }

    render() {
        return html`<section dir="auto"><slot></slot></section>`;
    }
}
