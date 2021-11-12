import {
    LitElement,
    html,
    css,
    customElement,
    property,
    internalProperty,
    PropertyValues,
} from "lit-element";
import "@elements/core/popups/popup-body";
import "@elements/core/buttons/rectangle";

const STR_CLICK_TO_CREATE = "Click to create a code";

@customElement("share-jig-gen-code-button")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: block;
                }
                button {
                    cursor: pointer;
                    width: 100%;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    column-gap: 12px;
                    height: 64px;
                    border-radius: 8px;
                    font-size: 16px;
                    background-color: var(--main-blue);
                    box-sizing: border-box;
                    color: #ffffff;
                    border: 0;
                }
                :host([disabled]) button {
                    background-color: var(--light-blue-3);
                    color: var(--dark-blue-2);
                    border: solid 1px var(--dark-blue-2);
                }
                .icon {
                    border: solid 1px #ffffff;
                    color: #ffffff;
                    height: 16px;
                    display: inline-block;
                    padding: 0 4px;
                }
                :host([disabled]) .icon {
                    border-color: var(--dark-blue-2);
                    color: var(--dark-blue-2);
                }
            `,
        ];
    }

    @property({ type: Boolean, reflect: true })
    disabled: boolean = false;

    render() {
        return html`
            <button>
                ${STR_CLICK_TO_CREATE}
                <span class="icon"> • • • • </span>
            </button>
        `;
    }
}
