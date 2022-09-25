import { css, html, LitElement } from "lit-element";
import { TemplateResult } from "lit-html";

export class PopupBase extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: block;
                    width: 800px;
                    border-radius: 50px;
                    overflow: hidden;
                    box-shadow: 0 3px 6px 0 rgba(0, 0, 0, 0.16);
                }
                .top-section {
                    height: 560px;
                    max-height: 70vh;
                    background-color: #192150d9;
                    display: grid;
                    align-items: center;
                    align-content: center;
                    justify-content: center;
                    row-gap: 36px;
                    text-align: center;
                }
                .actions {
                    height: 164px;
                    background-color: #ffffff;
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    column-gap: 50px;
                }
            `,
        ];
    }

    renderBase(body: () => TemplateResult) {
        return html`
            <div class="top-section">${body()}</div>
            <div class="actions">
                <slot name="actions"></slot>
            </div>
        `;
    }
}
