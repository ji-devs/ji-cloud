import { mediaUi, MEDIA_UI } from "@utils/path";
import { LitElement, html, css, customElement, property, unsafeCSS } from "lit-element";

const STR_TITLE = "Welcome to the Jigzi family!";
const STR_SUB = "You can now create, play, and share your content.";
const STR_SUBSUB = "We are here to help.";

const backgroundImage = mediaUi("entry/user/register-complete/background.webp");

@customElement("page-register-complete")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: grid;
                    justify-content: center;
                    align-items: start;
                    padding-top: 160px;
                    box-sizing: border-box;
                    height: 100vh;
                    background-image: url("${unsafeCSS(backgroundImage)}");
                    background-repeat: no-repeat;
                    background-size: cover;
                    background-position: center center;
                }
                .content {
                    width: 500px;
                    border-radius: 32px;
                    box-shadow: 0 3px 6px 0 rgba(0, 0, 0, 0.16);
                    padding: 64px;
                    background-color: #fff;
                }
                .title {
                    text-align: left;
                    margin-bottom: 56px;
                }
                h1 {
                    font-size: 32px;
                    font-weight: 900;
                    color: #5662a3;
                }
            `,
        ];
    }

    render() {
        return html`
            <div class="content">
                <div class="title">
                    <h1>${STR_TITLE}</h1>
                    <title-ji color="black">${STR_SUB}</title-ji>
                    <title-ji color="black">${STR_SUBSUB}</title-ji>
                </div>
                <slot name="button"></slot>
            </div>
        `;
    }
}
