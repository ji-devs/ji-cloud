import { LitElement, html, css, customElement } from "lit-element";
import "@elements/core/images/ui";

const STR_TITLE = "Like what you see?";
const STR_MESSAGE_1 = "To make changes, click back to any step.";
const STR_MESSAGE_2 = " If you are happy with your activity, click Done.";

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
                    font-weight: bold;
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
                .message p {
                    margin: 6px;
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
                <p>${STR_MESSAGE_1}</p>
                <p>${STR_MESSAGE_2}</p>
            </div>
            <div class="actions">
                <slot name="actions"></slot>
            </div>
            <slot name="popup"></slot>
        `;
    }
}
