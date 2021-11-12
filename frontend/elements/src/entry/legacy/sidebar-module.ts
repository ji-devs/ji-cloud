import { MEDIA_UI } from "@utils/path";
import { LitElement, html, css, customElement, property } from "lit-element";
import { nothing } from "lit-html";
import { colorStyles } from "@elements/_styles/colors";
import { arrayIndex } from "@utils/array";

@customElement("legacy-sidebar-module")
export class _ extends LitElement {
    static get styles() {
        return [
            colorStyles,
            css`
                section {
                    display: flex;
                    flex-direction: column;
                    cursor: pointer;
                }

                .caption {
                    text-align: center;
                    width: 100%;
                }

                .img {
                    padding: 5px;
                    width: 312px;
                    height: 234px;
                }

                .selected {
                    padding: 0;
                    border-radius: 5px;
                    border: yellow 5px solid;
                }
            `,
        ];
    }

    @property()
    jigId: string = "";

    @property()
    moduleId: string = "";

    @property({ type: Number })
    index: number = 0;

    @property({ type: Boolean })
    selected: boolean = false;

    // Define the element's template
    render() {
        const { selected, index } = this;

        return html`
            <section>
                <div class="img ${selected ? "selected" : ""}">
                    <slot name="img"></slot>
                </div>
                <div class="caption">${index + 1}</div>
            </section>
        `;
    }
}
