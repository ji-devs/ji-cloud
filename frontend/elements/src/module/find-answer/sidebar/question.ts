import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/buttons/fa-button";

@customElement("question-item")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    border-radius: 8px;
                    box-shadow: 0 3px 6px 0 rgba(0, 0, 0, 0.25);
                    display: grid;
                    grid-template-columns: 24px auto;
                    grid-template-rows: 1fr auto;
                    padding: 6px;
                    box-sizing: border-box;
                    background-color: var(--white);
                    align-items: center;
                }

                .header {
                    display: flex;
                    flex-direction: row;
                    align-items: center;
                }

                .header div {
                    display: flex;
                    flex-direction: row;
                    align-items: center;
                }

                .header ::slotted(img-ui) {
                    margin-left: 8px;
                    display: block;
                }

                .header > div.title {
                    flex: 1;
                }

                .header > div.actions {
                    align-self: flex-start;
                }

                .body {
                    grid-column: 1 / 3;
                }

                ::slotted(fa-button) {
                    padding: 0 6px;
                }
            `,
        ];
    }

    @property({ type: Boolean, reflect: true })
    tabbed: boolean = false;

    @property({ type: String })
    title: string = "";

    render() {
        return html`
            <slot name="toggle"></slot>
            <div class="header">
                <div class="title">
                    <slot name="title"></slot>
                </div>
                <div class="actions">
                    <slot name="edit-btn"></slot>
                    <slot name="menu"></slot>
                </div>
            </div>
            <div class="body">
                <slot></slot>
            </div>
        `;
    }
}
