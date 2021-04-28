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
                .first {
                    grid-template-columns: 214px 105px 125px;
                }
                .second {
                    grid-template-columns: 1fr 1fr;
                }
                .third {
                    grid-template-columns: 136px 90px 216px;
                }
                .fourth {
                    grid-template-columns: 1fr 1fr;
                }
            `,
        ];
    }
    render() {
        return html`
            <div class="row first">
                <slot name="hebrew-keyboard"></slot>
                <slot name="dicta"></slot>
                <slot name="sefaria"></slot>
            </div>
            <div class="row second">
                <slot name="element"></slot>
                <slot name="font-size"></slot>
            </div>
            <div class="row third">
                <slot name="style"></slot>
                <slot name="color"></slot>
                <slot name="justify"></slot>
            </div>
            <div class="row fourth">
                <slot name="font"></slot>
                <slot name="weight"></slot>
            </div>
        `;
    }
}
