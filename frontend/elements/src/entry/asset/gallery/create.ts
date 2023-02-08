import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/images/ui";

const STR_LABEL = "New";

@customElement("asset-gallery-create")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: inline-grid;
                    place-items: center;
                    align-content: center;
                    row-gap: 14px;
                    box-shadow: 0 3px 10px 0 rgba(0, 0, 0, 0.16);
                    border-radius: 16px;
                    cursor: pointer;
                    background-color: #fff;
                    height: 96px;
                    width: 120px;
                    box-sizing: border-box;
                }
                :host(:hover),
                :host(:focus) {
                    background-color: var(--light-blue-2);
                }
                img-ui {
                    height: 32px;
                    width: 32px;
                }
                .label {
                    color: #4f4f4f;
                    font-size: 14px;
                    font-weight: 600;
                }
            `,
        ];
    }

    @property()
    assetName = "";

    render() {
        return html`
            <img-ui path="entry/jig/gallery/add-icon.svg"></img-ui>
            <span class="label">${STR_LABEL} ${this.assetName}</span>
        `;
    }
}
