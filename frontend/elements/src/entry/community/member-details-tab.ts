import { LitElement, html, css, customElement, property } from "lit-element";

@customElement("community-member-details-tab")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    font-size: 16px;
                    font-weight: 500;
                    color: var(--dark-gray-5);
                    border-bottom: solid #fff 3px;
                    text-align: center;
                    padding: 0 40px;
                    display: inline-block;
                    cursor: pointer;
                }
                :host([active]) {
                    color: var(--main-blue);
                    border-color: var(--main-blue);
                }
            `,
        ];
    }

    @property({ type: Boolean, reflect: true })
    active: boolean = false;

    render() {
        return html`
            <slot></slot>
        `;
    }
}
