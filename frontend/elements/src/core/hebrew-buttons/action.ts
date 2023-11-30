import { LitElement, html, css, customElement, property } from "lit-element";

type Kind = "sefaria" | "dicta" | "keyboard";

@customElement("hebrew-inputs-action")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                button {
                    background-color: transparent;
                    border: 0;
                    cursor: pointer;
                    padding: 0;
                    display: grid;
                    place-content: center;
                    height: 34px;
                }
                button img-ui {
                    display: none;
                    height: 20px;
                }
                :host(:not([active]):not(:hover)) .img-default {
                    display: block;
                }
                :host(:not([active]):hover) .img-hover {
                    display: block;
                }
                :host([active]) .img-active {
                    display: block;
                }
            `,
        ];
    }

    @property({ reflect: true })
    kind: Kind = "keyboard";

    @property({ type: Boolean, reflect: true })
    active: boolean = false;

    firstUpdated() {
        // Required for Safari so that this element becomes focusable and can be used with relatedTarget in <input-textarea-content>.`onBlur`.
        // See https://stackoverflow.com/a/42764495/5253155
        this.tabIndex = -1;
    }

    render() {
        return html`
            <button type="button">
                <img-ui
                    class="img-default"
                    path="core/hebrew-buttons/${this.kind}.svg"
                ></img-ui>
                <img-ui
                    class="img-hover"
                    path="core/hebrew-buttons/${this.kind}-hover.svg"
                ></img-ui>
                <img-ui
                    class="img-active"
                    path="core/hebrew-buttons/${this.kind}-active.svg"
                ></img-ui>
            </button>
        `;
    }
}
