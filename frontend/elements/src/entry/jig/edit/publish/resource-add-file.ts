import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/images/ui";

const STR_UPLOAD_FILE = "Upload file";

@customElement("jig-edit-publish-resource-add-file")
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
                .body {
                    padding: 0 32px 32px 32px;
                    display: grid;
                    row-gap: 8px;
                }
                ::slotted(input-file) {
                    border: dashed green 1px;
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
                    ${STR_UPLOAD_FILE}
                </h3>
                <div slot="body" class="body">
                    <label>
                        <slot name="input-file"></slot>
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
