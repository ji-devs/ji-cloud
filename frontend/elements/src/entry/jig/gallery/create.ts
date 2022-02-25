import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/images/ui";

const STR_LABEL = "New JIG";

@customElement("jig-gallery-create")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    border: solid 4px var(--dark-blue-1);
                    display: inline-grid;
                    box-shadow: 0 3px 10px 0 rgba(0, 0, 0, 0.16);
                    border-radius: 16px;
                    padding: 44px;
                    place-items: center;
                    row-gap: 15px;
                    cursor: pointer;
                    background-color: #fff;
                    height: 176px;
                    width: 232px;
                    box-sizing: border-box;
                }
                :host(:hover),
                :host(:focus) {
                    background-color: var(--light-blue-2);
                }
                .label {
                    color: #4f4f4f;
                    font-weight: 600;
                }
            `,
        ];
    }

    render() {
        return html`
            <img-ui path="entry/jig/gallery/add-icon.svg"></img-ui>
            <span class="label">${STR_LABEL}</span>
        `;
    }
}
