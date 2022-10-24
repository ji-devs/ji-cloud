import { LitElement, html, css, customElement } from "lit-element";
import "@elements/core/images/ui";

const STR_TITLE = "Great design";
const STR_MESSAGE = "To make changes, click back to any step. If you are happy with the activity you just saw, click done. Your work is auto-saved.";

@customElement("preview-body")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    width: 90%;
                    max-width: 508px;
                    display: flex;
                    flex-direction: column;
                    grid-gap: 32px;
                    align-items: center;
                    margin: 20px 0;
                    margin-top: 71px;
                }
                img-ui {
                    width: 178px;
                    display: grid;
                    align-content: center;
                }
                .title {
                    font-size: 28px;
                    text-align: center;
                    color: var(--dark-blue-4);
                }
                .message {
                    text-align: center;
                    line-height: 1.18;
                    font-size: 18px;
                    color: var(--dark-blue-4);
                    margin: 0;
                }
                .actions {
                    display: flex;
                    flex-direction: row;
                    justify-content: space-between;
                    width: 66%;
                }
            `,
        ];
    }

    render() {
        return html`
                <img-ui
                    path="module/_common/edit/post-preview/splash.png"
                ></img-ui>
                <div class="title">${STR_TITLE}</div>
                <div class="message">
                    ${STR_MESSAGE}
                </div>
                <div class="actions">
                    <slot name="actions"></slot>
                </div>
        `;
    }
}
