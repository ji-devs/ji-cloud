import { css, html, LitElement } from "lit-element";
import { TemplateResult } from "lit-html";

export class PopupBase extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    border-radius: 14px;
                    overflow: hidden;
                    box-shadow: 0 3px 6px 0 rgba(0, 0, 0, 0.16);
                    display: grid;
                    max-width: 70vw;
                    max-height: 92vh;
                    width: 300px;
                    grid-template-rows: 2fr 1fr;
                }
                @media (min-width: 1024px) {
                    :host {
                        width: 800px;
                        grid-template-rows: 1fr 164px;
                    }
                }
                .top-section {
                    background-color: #192150f5;
                    display: grid;
                    align-items: center;
                    align-content: center;
                    justify-content: center;
                    text-align: center;
                }
                @media (min-width: 1024px) {
                    .top-section {
                        row-gap: 36px;
                    }
                }
                .actions {
                    background-color: #ffffff;
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    column-gap: 30px;
                }
                @media (min-width: 1024px) {
                    .actions {
                        row-gap: 50px;
                    }
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
