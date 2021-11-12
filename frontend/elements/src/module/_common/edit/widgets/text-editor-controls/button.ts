import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/images/ui";

export type Kind =
    | "h1"
    | "h2"
    | "p1"
    | "p2"
    | "bold"
    | "italic"
    | "underline"
    | "align-left"
    | "align-center"
    | "align-right"
    | "color"
    | "highlight-color"
    | "box-color"
    | "indent"
    | "outdent";

@customElement("text-editor-controls-button")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    border-radius: 50%;
                    font-size: 18px;
                    height: 36px;
                    width: 36px;
                    line-height: 36px;
                    text-align: center;
                    font-family: "Poppins", sans-serif;
                    color: var(--dark-gray-6);
                    display: flex;
                    font-weight: 500;
                }
                :host(:hover) {
                    background-color: var(--light-blue-1);
                }
                :host([active]) {
                    background-color: var(--main-blue);
                }
                button {
                    border: 0;
                    background-color: transparent;
                    padding: 0;
                    height: inherit;
                    width: inherit;
                    font-size: inherit;
                    line-height: inherit;
                    font-family: inherit;
                    font-weight: inherit;
                    cursor: pointer;
                }
            `,
        ];
    }

    @property()
    kind: Kind = "h1";

    @property({ type: Boolean, reflect: true })
    active = false;

    render() {
        const path = `module/_common/edit/widgets/sidebar/text-editor-controls/${this.kind.toLowerCase()}${
            this.active ? "-active" : ""
        }.svg`;
        return html`
            <button>
                <img-ui path="${path}"></img-ui>
            </button>
        `;
    }
}
