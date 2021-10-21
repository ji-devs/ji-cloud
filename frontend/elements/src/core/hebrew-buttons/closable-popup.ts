import { LitElement, html, css, customElement, property } from "lit-element";

@customElement("hebrew-inputs-closable-popup")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                button {
                    position: absolute;
                    background-color: var(--main-blue);
                    color: #ffffff;
                    height: 32px;
                    width: 32px;
                    font-size: 26px;
                    border-radius: 50%;
                    transform: translate(-50%, -50%);
                    left: 100%;
                    z-index: 1;
                    border: 0;
                    cursor: pointer;
                }
            `,
        ];
    }

    @property({ type: Boolean, reflect: true })
    active: boolean = false;

    private close() {
        this.dispatchEvent(new Event("close"));
    }

    render() {
        return html`
            <button type="button" @click=${this.close}>&times;</button>
            <slot></slot>
        `;
    }
}
