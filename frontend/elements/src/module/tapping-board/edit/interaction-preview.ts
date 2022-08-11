import { LitElement, html, css, customElement, property } from "lit-element";

const STR_LABEL: string = "Preview";

@customElement("interaction-preview-action")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                button {
                    height: 40px;
                    border: 0;
                    box-sizing: border-box;
                    background-color: var(--dark-blue-3);
                    color: #fff;
                    padding: 0 24px;
                    border-radius: 20px;
                    display: flex;
                    align-items: center;
                    column-gap: 12px;
                    font-family: Poppins;
                    font-size: 16px;
                    font-weight: 500;
                    cursor: pointer;
                    transition: filter 100ms;
                }
                :host(:not([disabled])) button {
                    filter: opacity(85%);
                }
                :host(:not([disabled])) button:hover {
                    filter: opacity(100%);
                }
                :host([disabled]) button {
                    background-color: var(--light-gray-4);
                }
                img-ui {
                    display: inline-block;
                    height: 18px;
                }
            `,
        ];
    }

    @property({ type: Boolean, reflect: true })
    disabled: boolean = false;

    render() {
        const iconUrl = `action-preview${
            this.disabled ? "-disabled" : ""
        }`;

        return html`
            <button ?disabled="${this.disabled}">
                ${STR_LABEL}
                <img-ui
                    path="module/_common/edit/widgets/sidebar/icons/${iconUrl}.svg"
                ></img-ui>
            </button>
        `;
    }
}

