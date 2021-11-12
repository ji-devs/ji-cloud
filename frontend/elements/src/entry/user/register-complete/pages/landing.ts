import { MEDIA_UI } from "@utils/path";
import { LitElement, html, css, customElement, property } from "lit-element";

const STR_TITLE = "Welcome to the Jigzi family!";
const STR_SUB = "You can now create, play, and share your content.";
const STR_SUBSUB = "We are here to help.";

@customElement("page-register-complete")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: block;
                    height: 100vh;
                    overflow-y: hidden;
                }
                .content {
                    display: flex;
                    justify-content: center;
                    width: 100%;
                    flex-direction: column;
                    align-items: center;
                    margin-bottom: 80px;
                }
                .footer {
                    width: 100%;
                    height: 100%;
                    background-color: #def4ff;
                    overflow: hidden;
                    display: flex;
                    justify-content: center;
                }

                .content-inner {
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
                <div class="content-inner">
                    <div class="title">
                        <h1>${STR_TITLE}</h1>
                        <title-ji size="subMedium" color="black"
                            >${STR_SUB}</title-ji
                        >
                        <title-ji size="subMedium" color="black"
                            >${STR_SUBSUB}</title-ji
                        >
                    </div>
                    <slot name="button"></slot>
                </div>
            </div>
            <div class="footer">
                <img-ui
                    path="entry/user/register-complete/jigglings.png"
                ></img-ui>
            </div>
        `;
    }
}
