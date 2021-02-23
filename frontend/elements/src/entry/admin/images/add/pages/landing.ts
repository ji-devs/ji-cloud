import { LitElement, html, css, customElement, property } from 'lit-element';
import "@elements/core/titles/ji";
import "@elements/core/titles/variants/underlined-title";
import "@elements/core/inputs/search";
import { nothing } from "lit-html";
import "@elements/entry/admin/images/base-page";

export type ImageKind = "sticker" | "canvas";

const STR_TITLE = "Add Images";
const STR_SELECT = "Select an image kind:";
const STR_OPTION_STICKER = "Sticker";
const STR_OPTION_CANVAS= "Canvas";

@customElement('image-add-page')
export class _ extends LitElement {
    static get styles() {
        return [css`
        .button {
            margin-top: 32px;
        }
    `];
    }

    onRadioChange(evt:any) {
        this.imageKind = evt.target.value;

        this.dispatchEvent(
            new CustomEvent("custom-change", {
                detail: { value: this.imageKind},
            })
        );
    }

    @property()
    imageKind: ImageKind = "sticker";

    render() {
        
        const {imageKind} = this;

        return html`
            <image-page title="${STR_TITLE}">
                <div class="button">
                    <slot name="button"></slot>
                </div>
                <div>
                    <p>${STR_SELECT}</p>

                    <div>
                        <label>
                            <input @change=${this.onRadioChange} type="radio" name="img_kind" value="sticker" .checked=${imageKind === "sticker"} />
                            ${STR_OPTION_STICKER}
                        </label>
                    </div>
                    <div>
                        <label>
                            <input @change=${this.onRadioChange} type="radio" name="img_kind" value="canvas" .checked=${imageKind === "canvas"} />
                            ${STR_OPTION_CANVAS}
                        </label>
                    </div>
                </div>
            </image-page>
        `;
    }
}
