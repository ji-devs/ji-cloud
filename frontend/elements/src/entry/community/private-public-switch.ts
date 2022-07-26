import { LitElement, html, css, customElement, property } from "lit-element";

const STR_PRIVATE = "Private";
const STR_PUBLIC = "Public";

@customElement("community-private-public-switch")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    width: 180px;
                    height: 24px;
                    display: grid;
                    grid-template-columns: 50% 50%;
                    border-radius: 8px;
                    overflow: hidden;
                    font-size: 14px;
                    font-weight: 500;
                }
                button {
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    column-gap: 6px;
                    cursor: pointer;
                    background-color: #e7edf0;
                    color: var(--dark-gray-3);
                    border: none;
                }
                :host(:not([isPublic])) .private,
                :host([isPublic]) .public {
                    color: #fff;
                    background-color: #5590fc;
                }
            `,
        ];
    }

    @property({ type: Boolean, reflect: true })
    isPublic = false;

    private onChange(isPublic: boolean) {
        this.isPublic = isPublic;
        this.dispatchEvent(
            new CustomEvent("custom-toggle", {
                detail: { value: isPublic },
            })
        );
    }

    render() {
        return html`
            <button class="private" @click=${() => this.onChange(false)}>
                <fa-icon icon="fa-light fa-eye-slash"></fa-icon>
                ${STR_PRIVATE}
            </button>
            <button class="public" @click=${() => this.onChange(true)}>
                <fa-icon icon="fa-light fa-eye"></fa-icon>
                ${STR_PUBLIC}
            </button>
        `;
    }
}
