import { LitElement, html, css, customElement } from "lit-element";

@customElement("footer-contact")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                p {
                    color: var(--dark-gray-5);
                    font-size: 14px;
                }
            `,
        ];
    }

    render() {
        const STR_CONTACT = "If you need help, contact us at: ";
        return html`
            <p>
                ${STR_CONTACT}
                <button-rect
                    color="blue"
                    kind="text"
                    href="mailto:info@jigzi.org"
                >
                    info@jigzi.org
                </button-rect>
            </p>
        `;
    }
}
