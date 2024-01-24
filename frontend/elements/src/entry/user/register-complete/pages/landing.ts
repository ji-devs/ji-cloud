import { mediaUi } from "@utils/path";
import { LitElement, html, css, customElement, property, unsafeCSS } from "lit-element";

const backgroundImage = mediaUi("entry/user/register-complete/background.webp");

@customElement("page-register-complete")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: grid;
                    box-sizing: border-box;
                    height: 100dvh;
                    background-image: url("${unsafeCSS(backgroundImage)}");
                    background-repeat: no-repeat;
                    background-size: cover;
                    background-position: center center;
                }
                .logo {
                    grid-column: 1;
                    grid-row: 1;
                    margin: 30px;
                    align-self: start;
                    justify-self: center;
                }
                @media (min-width: 1024px) {
                    .logo {
                        justify-self: start;
                        display: block;
                    }
                }
                .content {
                    grid-column: 1;
                    grid-row: 1;
                    margin-top: 160px;
                    justify-self: center;
                    align-self: start;
                    width: 710px;
                    max-width: 95vw;
                    border-radius: 32px;
                    box-shadow: 0 3px 6px 0 rgba(0, 0, 0, 0.16);
                    padding: 64px;
                    box-sizing: border-box;
                    background-color: #fff;
                }
                .title {
                    text-align: left;
                    margin-bottom: 56px;
                }
                ::slotted(h1) {
                    font-size: 32px;
                    font-weight: 900;
                    color: #5662a3;
                }
                ::slotted(h2) {
                    font-size: 16px;
                    color: rgb(74, 74, 74);
                    font-weight: 500;
                }
                .actions {
                    display: flex;
                    gap: 24px;
                }
                ::slotted(p) {
                    font-size: 14px;
                    color: var(--dark-gray-6);
                }
            `,
        ];
    }

    render() {
        return html`
            <a class="logo" href="/">
                <img-ui path="core/page-header/logo.svg"></img-ui>
            </a>
            <div class="content">
                <div class="title">
                    <slot name="headings"></slot>
                </div>
                <div class="actions">
                    <slot name="actions"></slot>
                </div>
                <slot name="help"></slot>
            </div>
        `;
    }
}
