import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/popups/popup-body";
import { scrollbarStyles } from "@elements/_styles/scrollbar";

const STR_HEADER = "Themes";

@customElement("jig-settings-themes")
export class _ extends LitElement {
    static get styles() {
        return [
            scrollbarStyles,
            css`
                popup-body {
                    width: 514px;
                    display: block;
                    padding: 15px 0;
                }
                .themes {
                    display: grid;
                    grid-template-columns: repeat(2, auto);
                    row-gap: 3px;
                    justify-content: space-evenly;
                    height: 400px;
                    overflow: auto;
                }
            `,
        ];
    }

    render() {
        return html`
            <popup-body>
                <slot name="back" slot="back"></slot>
                <slot name="close" slot="close"></slot>
                <h2 slot="heading">${STR_HEADER}</h2>
                <div class="themes scrollbar" slot="body">
                    <slot></slot>
                </div>
            </popup-body>
        `;
    }
}
