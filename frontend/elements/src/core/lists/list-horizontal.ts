import { MEDIA_UI } from "@utils/path";
import { LitElement, html, css, customElement, property } from "lit-element";
@customElement("list-horizontal")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                ul {
                    padding-left: 0;
                    display: flex;
                    flex-wrap: wrap;
                }

                .label {
                    color: #5590fc;
                    font-weight: 500;
                    margin-top: 0;
                }
            `,
        ];
    }

    @property()
    label: string = "";

    render() {
        const { label } = this;

        return html`
            <main>
                <p class="label">${label}</p>
                <ul>
                    <slot></slot>
                </ul>
            </main>
        `;
    }
}
