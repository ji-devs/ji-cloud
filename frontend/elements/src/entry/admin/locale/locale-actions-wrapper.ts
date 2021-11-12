import { LitElement, html, css, customElement } from "lit-element";

@customElement("locale-actions-wrapper")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: flex;
                    column-gap: 4px;
                    padding: 4px;
                }
                span {
                    content: "|";
                }
            `,
        ];
    }

    render() {
        return html`
            <slot name="first"></slot>
            <span>|</span>
            <slot name="second"></slot>
        `;
    }
}
