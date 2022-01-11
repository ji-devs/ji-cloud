import { LitElement, html, css, customElement, property } from "lit-element";

@customElement("module-header")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                .topRight {
                    position: absolute;
                    top: 21px;
                    right: 40px;
                    display: flex;
                    gap: 24px;
                    align-items: center;
                }
                .title {
                    margin-top: 90px;
                    font-size: 32px;
                    font-weight: 900;
                    letter-spacing: -0.32px;
                    text-align: left;
                    color: var(--dark-blue-4);
                }
                :host([subtitle]) .title {
                    margin-top: 30px;
                }
                .subtitle {
                    margin: 10px 0 10px 0;
                    font-family: Poppins;
                    font-size: 24px;
                    font-weight: 500;
                    letter-spacing: normal;
                    text-align: left;
                    color: var(--dark-blue-4);
                }
            `,
        ];
    }

    @property({ type: String })
    headerTitle: string = "";

    @property({ type: String, reflect: true })
    subtitle?: string;

    render() {
        return html`
            <section>
                <div class="topRight">
                    <slot name="controller"></slot>
                    <slot name="help"></slot>
                </div>
                <div class="title">${this.headerTitle}</div>
                ${this.subtitle && html`
                    <div class="subtitle">${this.subtitle}</div>
                `}
                <slot></slot>
            </section>
        `;
    }
}
