import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/popups/popup-body";
import "@elements/core/buttons/rectangle";

const STR_STUDENTS_HEADER = "Share with Students";
const STR_STUDENTS_URL_LABEL = "Ask the students to go to:";
const STR_STUDENTS_URL_LINK = "Go to site";
const STR_STUDENTS_CODE_LABEL = "Student code:";
const STR_STUDENTS_CODE_VALID_UNTIL = "Valid for a week until NOV 10th";

@customElement("share-jig-students")
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
                input {
                    background-color: #f7f7f7;
                    border-radius: 8px;
                    padding: 14px 18px;
                    font-size: 16px;
                    font-weight: 500;
                    color: var(--dark-gray-6);
                    border: 0;
                    width: 100%;
                    box-sizing: border-box;
                }
                .field-url .under {
                    display: flex;
                    justify-content: flex-end;
                    align-items: center;
                    column-gap: 8px;
                }
                .field-url .under .divider {
                    height: 1em;
                    width: 2px;
                    background-color: var(--main-blue);
                }
                .field-code input {
                    font-size: 48px;
                }
                .field-code .under {
                    display: flex;
                    justify-content: space-between;
                }
                .field-code .valid-until {
                    color: #4a4a4a;
                    font-size: 14px;
                }
            `,
        ];
    }

    @property()
    url: string = "";

    @property()
    code: string = "";

    render() {
        return html`
            <popup-body>
                <slot slot="back" name="back"></slot>
                <slot slot="close" name="close"></slot>
                <h3 slot="heading">${STR_STUDENTS_HEADER}</h3>
                <div slot="body" class="body">
                    <div class="field-url">
                        <label>
                            ${STR_STUDENTS_URL_LABEL}
                            <input readonly value="${this.url}">
                        </label>
                        <div class="under">
                            <a href="${this.url}"><button-rect kind="text">${STR_STUDENTS_URL_LINK}</button-rect></a>
                            <span class="divider"></span>
                            <slot name="copy-url"></slot>
                        </div>
                    </div>
                    <div class="field-code">
                        <label>
                            ${STR_STUDENTS_CODE_LABEL}
                            <input readonly value="${this.code}">
                        </label>
                        <div class="under">
                            <span class="valid-until">${STR_STUDENTS_CODE_VALID_UNTIL}</span>
                            <slot name="copy-code"></slot>
                        </div>
                    </div>
                </div>
            </popup-body>
        `;
    }
}
