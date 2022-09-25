import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/images/ui";

const STR_ENTER_URL = "Enter URL";
const STR_ENTER_LINK_HERE = "Enter your link here";

@customElement("jig-edit-publish-resource-add-link")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                popup-body {
                    box-shadow: 0 3px 16px 0 rgba(0, 0, 0, 0.16);
                    border-radius: 16px;
                    width: 420px;
                    background-color: #ffffff;
                }
                ::slotted([slot=close]) {
                    font-size: 18px;
                }
                h3 {
                    display: flex;
                    align-items: center;
                    column-gap: 6px;
                }
                h3 fa-icon {
                    font-size: 20px;
                }
                .body {
                    padding: 0 32px 32px 32px;
                    display: grid;
                }
                label {
                    display: grid;
                    font-size: 16px;
                    font-weight: 500;
                    color: #4a4a4a;
                    row-gap: 8px;
                }
                ::slotted(textarea) {
                    margin-top: 8px;
                    background-color: var(--light-blue-2);
                    border-radius: 8px;
                    padding: 8px 15px;
                    font-size: 16px;
                    font-weight: 500;
                    color: var(--dark-gray-6);
                    border: 0;
                    height: 100px;
                    resize: none;
                }
                ::slotted(textarea[error]) {
                    outline-color: var(--red-alert);
                    background-color: var(--light-red-alert);
                }
                ::slotted([slot=type]) {
                    margin-top: 24px;
                }
                .actions {
                    margin-top: 32px;
                    display: grid;
                    grid-auto-flow: column;
                    justify-content: space-between;
                }
            `,
        ];
    }

    @property()
    value: string = "";

    render() {
        return html`
            <popup-body>
                <slot slot="back" name="back"></slot>
                <slot slot="close" name="close"></slot>
                <h3 slot="heading">
                    <fa-icon icon="fa-regular fa-link-simple"></fa-icon>
                    ${STR_ENTER_URL}
                </h3>
                <div slot="body" class="body">
                    <label>
                        ${STR_ENTER_LINK_HERE}
                        <slot name="textarea"></slot>
                        <slot name="type"></slot>
                    </label>
                    <div class="actions">
                        <slot name="actions"></slot>
                    </div>
                </div>
            </popup-body>
        `;
    }
}
