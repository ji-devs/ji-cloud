import { LitElement, html, css, customElement, property } from 'lit-element';

export type Color = "red" | "blue" | "green";
export type Size = "small" | "medium" | "large";
export type Kind = "filled" | "text" | "outline";

@customElement("button-rect")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    border-radius: 24px;
                    border: none;
                    cursor: pointer;
                    font-size: 16px;
                    display: inline-grid;
                    place-content: center;
                    background-color: transparent;
                    box-sizing: border-box;
                    padding: 0;
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

                :host([kind=filled]) {
                    background-color: var(--color);
                    color: #ffffff;
                }
                :host([kind=outline]) {
                    color: var(--color);
                    border: solid 1px var(--color);
                }
                :host([kind=text]) {
                    color: var(--color);
                }

                :host([bold]) {
                    font-weight: bold;
                }

                :host([italic]) {
                    font-style: italic;
                }

                :host([size=small]:not([kind=text])) {
                    padding: 8px 22px;
                }
                :host([size=medium]:not([kind=text])) {
                    padding: 12px 24px;
                }
                :host([size=large]:not([kind=text])) {
                    padding: 16px 40px;
                }

                :host([disabled][kind=filled]) {
                    background-color: var(--light-gray-4);
                    color: #ffffff;
                }
                :host([disabled][kind=outline]) {
                    color: var(--light-gray-4);
                    border: solid 1px var(--light-gray-4);
                }
                :host([disabled][kind=text]) {
                    color: var(--light-gray-4);
                }

                button {
                    all: inherit;
                    display: contents;
                }
            `
        ];
    }

    @property({ reflect: true })
    size: Size = "medium";

    @property({ reflect: true })
    color: Color = "red";

    @property({ reflect: true })
    kind: Kind = "filled";

    @property({ type: Boolean, reflect: true })
    bold: boolean = false;

    @property({ type: Boolean, reflect: true }) // needed?
    italic: boolean = false;

    @property({ type: Boolean, reflect: true })
    disabled: boolean = false;

    @property({ type: Boolean })
    submit: boolean = false;

    connectedCallback() {
        super.connectedCallback();
        this.addEventListener("click", this.onClick, true);
    }
    disconnectedCallback() {
        super.disconnectedCallback();
        this.removeEventListener("click", this.onClick);
    }
    onClick(e: MouseEvent) {
        if(this.disabled)
            e.stopPropagation();
    }

    render() {
        return html`
            <button type="${this.submit ? 'submit' : 'button' }" ?disabled="${this.disabled}">
                <slot></slot>
            </button>
        `;
    }
}
