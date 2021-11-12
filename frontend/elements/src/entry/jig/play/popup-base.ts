import { css, html, LitElement, property } from "lit-element";
import { nothing, TemplateResult } from "lit-html";

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
                    min-height: 560px;
                    background-color: #192150d9;
                    display: grid;
                    align-items: center;
                    align-content: center;
                    justify-content: center;
                    row-gap: 36px;
                    text-align: center;
                }
                ::slotted([slot="actions"]) {
                    height: 164px;
                    background-color: #ffffff;
                    display: grid;
                    place-content: center;
                }
            `,
        ];
    }

    renderBase(body: () => TemplateResult) {
        return html`
            <div class="top-section">${body()}</div>
            <slot name="actions"></slot>
        `;
    }
}
