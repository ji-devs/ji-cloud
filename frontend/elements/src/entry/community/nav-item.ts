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
                    justify-items: center;
                }
                ::slotted(*) {
                    width: 56px;
                    height: 56px;
                    border-radius: 50%;
                    overflow: hidden;
                }
                ::slotted(fa-icon) {
                    border: solid 2px #ffffff;
                    background-color: #fee0e2;
                    color: #f2777f;
                    display: inline-grid;
                    place-content: center;
                    font-size: 28px;
                    box-sizing: border-box;
                }
                :host([active]) ::slotted(*) {
                    box-shadow: 0 3px 6px 0 rgba(0, 0, 0, 0.16);
                }
                :host([active]) ::slotted(fa-icon) {
                    background-color: #ffffff;
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
