import { LitElement, html, css, customElement, property } from 'lit-element';

export type Image = "" | "play" | "askforhelp" | "tryagain";

@customElement('studentcode-section')
export class _ extends LitElement {
    static get styles() {
        return [css`
            .inside-wrapper {
                display: flex;
                margin-left: 170px;
                margin-top: 130px;
            }

            main {
                width: 760px;
                height: 462px;
                position: relative;
            }

            ::slotted([slot=title]) {
                position: relative;
                text-align: center;
                top: 86px;
            }

            .img {
                bottom: -5px;
                left: 0px;
                position: absolute;
            }

            ::slotted([slot=baloon]) {
                width: 216.6px;
                height: 143.1px;
                bottom: 40px;
                left: 120px;
                position: absolute;
            }

            ::slotted([slot=square]) {
                margin-left: 32px;
                display: block;
            }
        `];
    }

    @property()
    kindimage: Image = "";

    render() {
        const { kindimage } = this;

        const path = kindimage === "play" ? " Illustration_JIG_Sad_1.png"
            : kindimage === "askforhelp" ? "Image_Jig_Studentcode_error2@2x.png"
            : kindimage === "tryagain" ? "Image_Jig_Studentcode_error1@2x.png"
            : "";

        return html`
            <main>
                <slot name="title"></slot>
                <div class="inside-wrapper">
                    <slot name="square"></slot>
                </div>
                <img-ui class="img" path="${path}"></img-ui>
            </main>
        `;
    }
}