import { LitElement, html, css, customElement } from "lit-element";
import { textToLineArray } from "@utils/text";

const STR_HEADER = "Great theme!";
const STR_PARAGRAPH = "Apply this theme to your whole JIG to set\nbackground, fonts and colors everywhere.";

@customElement("theme-selector-apply-popup")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: grid;
                    justify-content: center;
                    padding: 64px 10px;
                    box-shadow: 0 3px 16px 0 rgba(0, 0, 0, 0.16);
                    border-radius: 16px;
                    width: 800px;
                    max-width: 90vw;
                    background-color: #ffffff;
                }
                h2 {
                    font-size: 32px;
                    font-weight: bold;
                    text-align: center;
                    color: var(--dark-blue-5);
                    margin: 0;
                }
                p {
                    margin: 0;
                    margin-top: 16px;
                    font-size: 16px;
                    line-height: 1.5;
                    text-align: center;
                    color: var(--dark-gray-6);
                }
                img-ui {
                    margin-top: 48px;
                    max-width: 476px;
                }
                .actions {
                    margin-top: 56px;
                    display: grid;
                    grid-auto-flow: column;
                    justify-content: center;
                    column-gap: 48px;
                }
            `,
        ];
    }

    render() {
        return html`
            <h2>${STR_HEADER}</h2>
            <p>${
                textToLineArray(STR_PARAGRAPH).map(s => {
                    if(s === "\n") return html`<br>`;
                    else return s
                })
            }</p>
            <img-ui path="module/_common/edit/widgets/theme-selector/popup-image.webp"></img-ui>
            <div class="actions">
                <slot name="actions"></slot>
            </div>
        `;
    }
}
