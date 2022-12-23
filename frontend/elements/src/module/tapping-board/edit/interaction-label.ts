import { LitElement, html, css, customElement, property } from "lit-element";

export type mode = "default" | "active" | "success" | "done";

@customElement("tapping-board-interaction-label")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: flex;
                    flex-direction: column;
                    row-gap: 20px;
                    padding-top: 20px
                }
                .main-content {
                    border-radius: 16px;
                    background-color: var(--white);
                }
                .actions {
                    padding: 0;
                    display: flex;
                    flex-direction: row;
                    justify-content: space-around;
                }
                .actions ::slotted(*) {
                    grid-column: 1;
                    grid-row: 1;
                }
                ::slotted([slot="main-action"]) {
                    justify-self: center;
                }
            `,
        ];
    }

    @property({ type: String, reflect: true })
    mode: mode = "default";

    render() {
        return html`
            <div class="main-content">
                <slot></slot>
            </div>
            <div class="actions">
                <slot name="delete"></slot>
                <slot name="main-action"></slot>
            </div>
        `;
    }
}

