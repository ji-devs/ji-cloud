import { LitElement, html, css, customElement, property } from "lit-element";

@customElement("steps-nav")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: flex;
                    justify-content: space-between;
                    padding: 0 16px;
                }
                :host([dense]) {
                    padding: 0 6px;
                }
            `,
        ];
    }

    @property({ type: Boolean, reflect: true })
    dense: boolean = false;

    render() {
        return html` <slot></slot> `;
    }
}
