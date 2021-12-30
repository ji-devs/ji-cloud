import { LitElement, html, css, customElement } from "lit-element";
import "@elements/core/images/ui";

const STR_SENT = "I didn’t receive an email. Please send again.";

@customElement("button-email-send")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                .wrapper {
                    box-shadow: 2px 2px 3px 0 rgba(0, 0, 0, 0.16);
                    border-radius: 14px;
                    background-color: #72cb91;
                    padding: 16px 0 16px 22px;
                    align-items: center;
                    width: 374px;
                    justify-items: center;
                    cursor: pointer;
                }
                span {
                    display: flex;
                    align-items: center;
                    color: #ffffff;
                }
                img-ui {
                    margin-right: 16px;
                }
            `,
        ];
    }

    render() {
        return html`
            <div class="wrapper">
                <span class="flex items-center font-sans font-normal text-lg">
                    <img-ui path="entry/user/email-sent.svg" alt=""></img-ui>
                    ${STR_SENT}
                </span>
            </div>
        `;
    }
}
