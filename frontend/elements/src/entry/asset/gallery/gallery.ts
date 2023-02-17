import { LitElement, html, css, customElement, property, unsafeCSS, state } from "lit-element";
import "@elements/core/images/ui";
import { mediaUi } from "@utils/path";

const STR_MY_1 = "My ";
const STR_MY_2 = "s";

@customElement("asset-gallery")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: grid;
                    min-height: 100dvh;
                    grid-template-rows: auto 1fr;
                }
                .width-holder {
                    max-width: 1600px;
                    margin: 0 auto;
                }
                .top-section {
                    background-color: var(--light-blue-6);
                    display: grid;
                }
                .top-section ::slotted([slot=back]),
                .top-section .width-holder {
                    grid-row: 1;
                    grid-column: 1;
                }
                .top-section ::slotted([slot=back]) {
                    margin: 40px;
                    place-self: start;
                    z-index: 1;
                    font-size: 14px;
                    font-weight: 500;
                    color: var(--light-blue-3);
                    text-decoration: none;
                    display: flex;
                    column-gap: 8px;
                }
                .top-section .width-holder {
                    display: grid;
                    grid-template-columns: auto auto;
                    justify-content: space-between;
                    align-items: center;
                    padding: 60px 40px 20px 40px;
                    width: 100%;
                    box-sizing: border-box;
                }
                h1 {
                    font-size: 28px;
                    font-weight: 900;
                    color: var(--main-yellow);
                    margin: 0;
                    display: flex;
                    align-items: center;
                }
                h1 img-ui {
                    max-height: 70px;
                    max-width: 70px;
                }
                .bottom-section {
                    grid-column: 1 / -1;
                    background-image: url(${unsafeCSS(
                        mediaUi("entry/jig/gallery/background.png")
                    )});
                    background-size: 100%;
                }
                .bottom-section .width-holder {
                    padding: 40px;
                    display: grid;
                    row-gap: 48px;
                }
                .recent-top-line {
                    display: grid;
                    grid-template-columns: auto auto 224px;
                    column-gap: 32px;
                    align-items: center;
                }
                ::slotted([slot="filters"]) {
                    justify-self: end;
                    min-width: 300px;
                    --background-color: #ffffff;
                }
                .recent-items {
                    display: grid;
                    grid-template-columns: repeat(auto-fill, 216px);
                    gap: 34px;
                    justify-content: space-between;
                }
                ::slotted(a[slot=recent-items]) {
                    text-decoration: none;
                    color: inherit;
                }
                .load-more {
                    display: grid;
                    grid-template-columns: auto;
                    justify-content: center;
                }
            `,
        ];
    }

    @property()
    name: string = "";

    @property()
    assetDisplayName: string = "";

    @property()
    kind: string = "";

    render() {
        return html`
            <section class="top-section">
                <slot name="back"></slot>
                <div class="width-holder">
                    <h1 class="create-asset-header">
                        <img-ui path="entry/jig/gallery/${this.kind}-icon.webp"></img-ui>
                        ${STR_MY_1}${`${this.assetDisplayName}${STR_MY_2}`}
                    </h1>
                    <slot name="create-asset"></slot>
                </div>
            </section>
            <section class="bottom-section">
                <div class="width-holder">
                    <div class="recent-top-line">
                        <slot class="filters" name="filters"></slot>
                        <slot class="search-input" name="search-input"></slot>
                    </div>
                    <div class="recent-items">
                        <slot name="recent-items"></slot>
                    </div>
                    <div class="load-more">
                        <slot name="load-more"></slot>
                    </div>
                </div>
            </section>
        `;
    }
}
