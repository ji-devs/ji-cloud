import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/images/ui";

const STR_ADD_RESOURCES = "Add resources";

@customElement("jig-edit-publish-resource-add")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                popup-body {
                    box-shadow: 0 3px 16px 0 rgba(0, 0, 0, 0.16);
                    border-radius: 16px;
                    width: 420px;
                }
                .body {
                    padding: 0px 32px 44px;
                    display: grid;
                    row-gap: 6px;
                }
                ::slotted(button) {
                    all: unset;
                    cursor: pointer;
                    display: grid;
                    grid-template-columns: 18px 1fr;
                    column-gap: 8px;
                    font-size: 14px;
                }
                ::slotted(button:hover),
                ::slotted(button:hover) {
                    color: var(--main-blue)
                }
            `,
        ];
    }

    @property()
    label: string = "";

    render() {
        return html`
            <popup-body>
                <slot slot="close" name="close"></slot>
                <h2 slot="heading">${STR_ADD_RESOURCES}</h2>
                <div class="body" slot="body">
                    <slot name="options"></slot>
                </div>
            </popup-body>
        `;
    }
}
