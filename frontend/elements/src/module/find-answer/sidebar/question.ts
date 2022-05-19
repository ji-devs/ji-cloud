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
                    margin-right: 8px;
                }

                .header ::slotted(img-ui) {
                    margin-left: 8px;
                    display: block;
                }

                .header > div {
                    flex: 1;
                }

                .body {
                    grid-column-start: 2;
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
                <div>
                    <span>
                        <slot name="title"></slot>
                    </span>
                    <span>
                        <slot name="edit-btn"></slot>
                    </span>
                </div>
                <slot name="menu"></slot>
            </div>
            <div class="body">
                <slot></slot>
            </div>
        `;
    }
}
