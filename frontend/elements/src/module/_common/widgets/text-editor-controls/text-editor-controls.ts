import { LitElement, html, css, customElement } from "lit-element";


@customElement("text-editor-controls")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: grid;
                    row-gap: 22px;
                }
                .row {
                    display: grid;
                    column-gap: 22px;
                    justify-content: space-between;
                }
                .first, .second {
                    grid-template-columns: 296px 174px;
                }
                .third {
                    grid-template-columns: 136px 90px 216px;
                }
                .fourth {
                    grid-template-columns: 214px 105px 125px;
                }
            `,
        ];
    }
    render() {
        return html`
            <div class="row first">
                <slot name="font"></slot>
                <slot name="type"></slot>
            </div>
            <div class="row second">
                <slot name="weight"></slot>
                <slot name="font-size"></slot>
            </div>
            <div class="row third">
                <slot name="style"></slot>
                <slot name="color"></slot>
                <slot name="justify"></slot>
            </div>
            <div class="row fourth">
                <slot name="hewbrew-keyboard"></slot>
                <slot name="dicta"></slot>
                <slot name="sefaria"></slot>
            </div>
        `;
    }
}
