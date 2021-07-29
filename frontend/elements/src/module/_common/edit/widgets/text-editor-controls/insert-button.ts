import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/images/ui";

const STR_INSERT_TEXTBOX = "Insert textbox";

@customElement("text-editor-controls-insert-button")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                button {
                    padding: 0;
                    width: 100%;
                    height: 64px;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    column-gap: 10px;
                    border-radius: 8px;
                    border: 0;
                    background-color: var(--main-blue);
                    color: white;
                    cursor: pointer;
                }
                :host([disabled]) button {
                    border: solid 1px var(--dark-blue-2);
                    background-color: var(--light-blue-3);
                    color: var(--dark-blue-2);
                }
            `,
        ];
    }

    @property({type: Boolean, reflect: true})
    disabled = false;

    render() {
        const path = `module/_common/edit/widgets/sidebar/text-editor-controls/insert-text-${this.disabled ? 'blue' : 'white'}.svg`;
        return html`
            <button ?disabled=${this.disabled}>
                ${STR_INSERT_TEXTBOX}
                <img-ui path="${path}"></img-ui>
            </button>
        `;
    }
}
