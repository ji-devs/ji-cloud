import { LitElement, html, css, customElement } from "lit-element";

const STR_LABEL = "Place each item in their correct areas";

@customElement("module-sidebar-drag-prompt")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    width: 100%;
                    height: 100%;
                    display: flex;
                    flex-direction: column;
                    justify-content: center;
                    align-items: center;
                }
                img-ui {
                    width: 280px;
                }
                .label {
                    font-weight: 500;
                    color: var(--dark-gray-6);
                    text-align: center;
                    margin-top: 20px;
                    font-size: 14px;
                }
            `,
        ];
    }

    render() {
        return html`
            <img-ui
                path="module/_common/edit/widgets/sidebar/drag-prompt.svg"
            ></img-ui>
            <div class="label">${STR_LABEL}</div>
        `;
    }
}
