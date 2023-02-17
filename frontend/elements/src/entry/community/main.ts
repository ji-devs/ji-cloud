import { LitElement, html, css, customElement, property } from "lit-element";
import "../../core/page-footer/page-footer";

const STR_COMMUNITY = "Community";
const STR_PINCUS_PARTNERSHIP = "In partnership with the Pincus Fund";

@customElement("community-main")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
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
                    padding: 30px 24px;
                    row-gap: 30px;
                }
                @media (min-width: 1024px) {
                    header .width-holder {
                        row-gap: 0;
                        justify-content: space-between;
                        grid-template-columns: auto auto;
                    }
                }
                h2 {
                    margin: 0;
                    color: #fed758;
                    font-weight: 900;
                    font-size: 32px;
                    text-align: center;
                }
                @media (min-width: 1024px) {
                    h2 {
                        text-align: left;
                    }
                }
                ::slotted([slot=nav]) {
                    display: flex;
                    justify-content: center;
                    column-gap: 16px;
                }
                main {
                    padding: 30px 24px;
                    background-color: #fff6d9;
                }
                .pincus {
                    display: grid;
                    justify-items: center;
                    margin-top: 60px;
                    row-gap: 24px;
                }
                .pincus h4 {
                    font-size: 16px;
                    font-weight: normal;
                    color: #383838;
                }
                .pincus a img-ui {
                    width: 312px;
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
