import { LitElement, html, css, customElement, property } from "lit-element";

@customElement("community-nav-item")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                a {
                    display: grid;
                    row-gap: 8px;
                    text-decoration: none;
                }
                ::slotted(*) {
                    width: 56px;
                    height: 56px;
                    border-radius: 50%;
                }
                .label {
                    font-size: 14px;
                    color: #383838;
                }
                :host([active]) .label {
                    font-weight: 600;
                    color: #ffffff;
                }
            `,
        ];
    }

    @property({ type: Boolean, reflect: true })
    active: boolean = false;

    @property()
    href: string = "";

    @property()
    label: string = "";

    render() {
        return html`
            <a href=${this.href}>
                <slot></slot>
                <span class="label">${this.label}</span>
            </a>
        `;
    }
}
