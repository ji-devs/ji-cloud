import { LitElement, html, css, customElement, property } from 'lit-element';

export type Color = "grey" | "red" | "blue" | "green"; // grey and disabled have the same color
export type Size = "small" | "medium" | "large" | "x-large";
export type Kind = "rect" | "text" | "outline";

@customElement("button-main")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host([color=grey]) {
                    --color: var(--light-gray-4);
                }
                :host([color=red]) {
                    --color: #fd6b71;
                }
                :host([color=red]:hover) {
                    --color: #ed6065;
                }
                :host([color=blue]) {
                    --color: #5590fc;
                }
                :host([color=blue]:hover) {
                    --color: #387af4;
                }
                :host([color=green]) {
                    --color: #71cf92;
                }
                :host([color=green]:hover) {
                    --color: #46ba6f;
                }
                button {
                    border-radius: 24px;
                    border: none;
                    cursor: pointer;
                    font-size: 16px;
                    display: grid;
                    place-content: center;
                    background-color: transparent;
                    box-sizing: border-box;
                }
                :host([kind=rect]) button {
                    background-color: var(--color);
                    color: #ffffff;
                }
                :host([kind=outline]) button {
                    color: var(--color);
                    border: solid 1px var(--color);
                }
                :host([kind=text]) button {
                    color: var(--color);
                }
                :host([bold]) button {
                    font-weight: bold;
                }
                :host([italic]) button {
                    font-style: italic;
                }
                :host([size=small]) button {
                    padding: 8px 22px;
                }
                :host([size=medium]) button {
                    padding: 12px 24px;
                }
                :host([size=large]) button {
                    padding: 16px 40px;
                }
                /* :host([size=x-large]) button {
                    padding: 12px 24px;
                    font-size: 24px;
                } */
                :host button:disabled {
                    background-color: #a9b1b5;
                }
            `
        ];
    }

    @property({ reflect: true })
    size: Size = "medium";

    @property({ reflect: true })
    color: Color = "red";

    @property({ reflect: true })
    kind: Kind = "rect";

    @property({ type: Boolean, reflect: true })
    bold: boolean = false;

    @property({ type: Boolean, reflect: true }) // needed?
    italic: boolean = false;

    @property({ type: Boolean, reflect: true })
    disabled: boolean = false;

    @property({ type: Boolean })
    submit: boolean = false;

    render() {
        return html`
            <button type="${this.submit ? 'submit' : 'button' }" ?disabled="${this.disabled}">
                <slot></slot>
            </button>
        `;
    }
}
