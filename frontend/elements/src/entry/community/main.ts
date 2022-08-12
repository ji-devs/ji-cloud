import { LitElement, html, css, customElement, property } from "lit-element";
import "../../core/page-footer/page-footer";

const STR_COMMUNITY = "Community";
const STR_PINCUS_PARTNERSHIP = "in partnership with the Pincus Fund.";

@customElement("community-main")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    min-height: 100vh;
                    min-height: 100dvh;
                    display: grid;
                    grid-template-rows: auto auto 1fr;
                }
                .width-holder {
                    max-width: 1400px;
                    margin: auto;
                }
                header {
                    background-color: #f2777f;
                }
                header .width-holder {
                    display: grid;
                    justify-content: center;
                    padding: 40px 14px;
                    row-gap: 40px;
                }
                @media (min-width: 1920px) {
                    header .width-holder {
                        justify-content: space-between;
                        padding: 40px 30px;
                        row-gap: 10px;
                        grid-template-columns: auto auto;
                    }
                }
                h2 {
                    margin: 0;
                    color: #fed758;
                    font-weight: 900;
                    font-size: 36px;
                    text-align: center;
                }
                @media (min-width: 1920px) {
                    h2 {
                        font-size: 48px;
                        text-align: left;
                    }
                }
                ::slotted([slot=nav]) {
                    display: flex;
                    justify-content: center;
                    column-gap: 20px;
                }
                @media (min-width: 1920px) {
                    ::slotted([slot=nav]) {
                        column-gap: 40px;
                    }
                }
                main {
                    padding: 40px 30px;
                    background-color: #fff6d9;
                }
                .pincus {
                    display: grid;
                    justify-items: center;
                    margin-top: 80px;
                    row-gap: 30px;
                }
                .pincus h4 {
                    font-size: 18px;
                    font-weight: normal;
                    color: #383838;
                }
            `,
        ];
    }

    render() {
        return html`
            <slot name="jigzi-header"></slot>
            <header>
                <div class="width-holder">
                    <h2>${STR_COMMUNITY}</h2>
                    <slot name="nav"></slot>
                    <slot name="search-bar"></slot>
                </div>
            </header>
            <main>
                <div class="width-holder">
                    <slot></slot>
                </div>
                <div class="pincus">
                    <h4>${STR_PINCUS_PARTNERSHIP}</h4>
                    <a href="https://www.pincusfund.org/" target="_BLANK">
                        <img-ui path="entry/community/pincus-logo.webp"></img-ui>
                    </a>
                </div>
            </main>
            <page-footer></page-footer>
        `;
    }
}
