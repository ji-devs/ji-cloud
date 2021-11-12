import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/titles/ji";
import "@elements/core/titles/variants/underlined-title";
import "@elements/core/inputs/composed/search";
import { nothing } from "lit-html";

export type ImageKind = "sticker" | "canvas";

const STR_TITLE = "Add Images";
const STR_SELECT = "Select an image kind:";
const STR_OPTION_STICKER = "Sticker";
const STR_OPTION_CANVAS = "Canvas";

@customElement("image-add-page")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                aside {
                    display: flex;
                    justify-content: space-between;
                    border-bottom: solid 1px #e5e7ef;
                    padding-bottom: 29px;
                    margin-bottom: 29px;
                }
                aside .title {
                    font-size: 24px;
                    font-weight: 300;
                    font-stretch: normal;
                    font-style: normal;
                    line-height: 1.25;
                    letter-spacing: -0.24px;
                    text-align: left;
                    color: #000000;
                    margin-right: 10px;
                }

                aside .right {
                    display: flex;
                    align-items: center;
                    gap: 24px;
                }

                :host {
                    display: block;
                    margin-top: 29px;
                    padding-left: 40px;
                    padding-right: 40px;
                }
                .button {
                    margin-top: 32px;
                }
            `,
        ];
    }

    onRadioChange(evt: any) {
        this.imageKind = evt.target.value;

        this.dispatchEvent(
            new CustomEvent("custom-change", {
                detail: { value: this.imageKind },
            })
        );
    }

    @property()
    imageKind: ImageKind = "sticker";

    @property()
    query: string = "";
    render() {
        const { query, imageKind } = this;

        return html`
            <aside>
                <div class="title">${STR_TITLE}</div>
                <div class="right">
                    <input-search .value=${query}></input-search>
                </div>
            </aside>
            <article>
                <div class="button">
                    <slot name="button"></slot>
                </div>
                <div>
                    <p>${STR_SELECT}</p>

                    <div>
                        <label>
                            <input
                                @change=${this.onRadioChange}
                                type="radio"
                                name="img_kind"
                                value="sticker"
                                .checked=${imageKind === "sticker"}
                            />
                            ${STR_OPTION_STICKER}
                        </label>
                    </div>
                    <div>
                        <label>
                            <input
                                @change=${this.onRadioChange}
                                type="radio"
                                name="img_kind"
                                value="canvas"
                                .checked=${imageKind === "canvas"}
                            />
                            ${STR_OPTION_CANVAS}
                        </label>
                    </div>
                </div>
                <slot></slot>
            </article>
        `;
    }
}
