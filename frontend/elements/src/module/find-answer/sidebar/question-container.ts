import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/buttons/fa-button";

@customElement("question-container")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    scrollbar-width: thin;
                    scrollbar-color: #e7f0fe transparent;
                    padding: 0 8px;
                    display: flex;
                    flex-direction: column;
                    gap: 12px;
                    margin: 12px 0;
                }
            `,
        ];
    }

    render() {
        return html`
            <slot></slot>
        `;
    }
}
