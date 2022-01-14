import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/popups/popup-body";

const STR_SHARING_OPTIONS = "Sharing Options";

@customElement("share-jig-main")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                popup-body {
                    border-radius: 16px;
                    box-shadow: rgb(0 0 0 / 25%) 0px 3px 16px 0px;
                    background-color: #ffffff;
                }
                .body {
                    padding: 0px 24px 32px 24px;
                    display: grid;
                    width: 420px;
                }
            `,
        ];
    }

    render() {
        return html`
            <popup-body>
                <slot slot="close" name="close"></slot>
                <h3 slot="heading">${STR_SHARING_OPTIONS}</h3>
                <div class="body" slot="body">
                    <slot></slot>
                </div>
            </popup-body>
        `;
    }
}
