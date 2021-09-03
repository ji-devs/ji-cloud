import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/popups/popup-body";

const STR_EMBED_HEADER = "Embed this JIG";
const STR_EMBED_CODE = "Embed code:";

@customElement("share-jig-embed")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    background-color: #ffffff;
                }
                .body {
                    padding: 0 32px 32px 32px;
                    display: grid;
                    row-gap: 8px;
                    width: 420px;
                }
                label {
                    display: grid;
                }
                textarea {
                    background-color: var(--light-blue-2);
                    border-radius: 8px;
                    padding: 14px 18px;
                    font-size: 16px;
                    font-weight: 500;
                    color: var(--dark-gray-6);
                    border: 0;
                    height: 200px;
                    resize: none;
                }
                ::slotted([slot=copy]) {
                    place-self: end;
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
                <h3 slot="heading">${STR_EMBED_HEADER}</h3>
                <div slot="body" class="body">
                    <label>
                        ${STR_EMBED_CODE}
                        <textarea readonly>${this.value}</textarea>
                    </label>
                    <slot name="copy"></slot>
                </div>
            </popup-body>
        `;
    }
}
