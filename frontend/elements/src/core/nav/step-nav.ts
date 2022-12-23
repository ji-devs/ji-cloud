import { LitElement, html, css, customElement, property } from "lit-element";

@customElement("step-nav")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: contents;
                    --circle-size: 40px;
                }

                :host {
                    --color: var(--dark-gray-3);
                    --border-color: #e9eff8;
                }
                :host([completed]),
                :host([active]) {
                    --color: var(--light-blue-6);
                    --border-color: var(--light-blue-6);
                }
                section {
                    cursor: pointer;
                    display: flex;
                    flex-direction: column;
                    align-items: center;

                    width: calc(var(--circle-size) + 2px);
                    color: var(--color);
                }
                .circle {
                    border-radius: 9999px;
                    height: var(--circle-size);
                    width: var(--circle-size);
                    border-style: solid;
                    border-width: 1px;
                    border-color: var(--light-gray-1);
                    background-color: white;
                    justify-content: center;
                    align-items: center;
                    display: flex;
                    text-align: center;
                    border-color: var(--border-color);
                }
                :host([active]) .circle {
                    background-color: var(--color);
                    color: #fff;
                    font-weight: bold;
                }
                :host([active]) .label {
                    font-weight: bold;
                }

                p.label {
                    font-weight: 500;
                    letter-spacing: 0.14px;
                    text-align: center;
                    color: var(--color);
                    margin: 5px 0;
                    font-size: 12px;
                }
                :host([dense]) p.label {
                    font-size: 11px;
                }

                :host(:last-child) .line {
                    display: none;
                }
                .line {
                    width: 100%;
                    margin-top: 20px;
                    display: grid;
                }
                .line::after {
                    content: "";
                    display: inline-block;
                    background-color: var(--light-gray-1);
                    height: 2px;
                    width: calc(100% + var(--circle-size) + 2px);
                }
            `,
        ];
    }

    @property({ type: Number })
    number: number = 1;

    @property({ type: String })
    label: string = "";

    @property({ type: Boolean, reflect: true })
    completed: boolean = false;

    @property({ type: Boolean, reflect: true })
    active: boolean = false;

    @property({ type: Boolean, reflect: true })
    dense: boolean = false;

    render() {
        return html`
            <section>
                <div class="circle">${this.number}</div>
                <p class="label">${this.label}</p>
            </section>
            <div class="line"></div>
        `;
    }
}
