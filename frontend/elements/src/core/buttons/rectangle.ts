import { LitElement, html, css, customElement, property } from 'lit-element';
import { ifDefined } from 'lit-html/directives/if-defined';

export type Color = "red" | "blue" | "green" | "darkGray";
export type Size = "small" | "medium" | "large";
export type Kind = "filled" | "text" | "outline";

/*
Some thoughts, this comment can be removed once this is resolved
problem: wanna be able to quickly style the inner element (button and a) without part
    solution 1) make inner display contents so that you can put the styling on the :host
        problem: in the case of a the link wont include the paddings on the side
    solution 2) make inner inherit all styles from parent
        problem: things like display, or percentage are going to be calculated for both :host and inner
    solution 3) hybrid, make :host display contents and have inner inherit all properties from parent except display that should be set with a custom property
        problem: will have to use this custom display property
*/

@customElement("button-rect")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    border-radius: 24px;
                    border: none;
                    cursor: pointer;
                    display: inline-grid;
                    place-content: center;
                    background-color: transparent;
                    box-sizing: border-box;
                    padding: 0;
                    user-select: none;
                    font-size: 14px;
                }
                @media (min-width: 1920px) {
                    :host {
                        font-size: 16px;
                    }
                }

                :host([disabled]) {
                    pointer-events: none;
                }

                :host([color=darkGray]) {
                    /* Just taken from the first time it was needed... */
                    color: #4a4a4a;
                }
                :host([color=lightGray]) {
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

                :host([color=blue]:hover), :host([hoverColor=blue]:hover) {
                    --color: #387af4;
                }
                :host([color=green]) {
                    --color: #71cf92;
                }
                :host([color=green]:hover) {
                    --color: #46ba6f;
                }

                :host([color=orange]) {
                    --color: #fc7551;
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

                :host([size=small]:not([kind=text])) .inner {
                    padding: 8px 22px;
                }
                :host([size=medium]:not([kind=text])) .inner {
                    padding: 12px 24px;
                }
                :host([size=large]:not([kind=text])) .inner {
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

                button, a {
                    all: unset;
                    color: inherit;
                    display: flex;
                    column-gap: 6px;
                }
            `
        ];
    }

    @property({ reflect: true })
    size: Size = "medium";

    @property({ reflect: true })
    color: Color = "red";

    @property({ reflect: true })
    hoverColor: Color | "" = "";

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

    @property()
    href?: string;

    @property()
    target?: string;

    connectedCallback() {
        super.connectedCallback();
        this.addEventListener("click", this.onClick, true);
    }
    private onClick(e: MouseEvent) {
        if(this.disabled)
            e.stopPropagation();
    }

    private renderButton() {
        return html`
            <button part="button" class="inner" type="${this.submit ? 'submit' : 'button' }" ?disabled="${this.disabled}">
                <slot></slot>
            </button>
        `;
    }
    private renderLink() {
        return html`
            <a part="a" class="inner" href=${this.href!} @click=${this.onClick} target=${ifDefined(this.target as any)}>
                <slot></slot>
            </a>
        `;
    }

    render() {
        return this.href === undefined ? this.renderButton() : this.renderLink();
    }
}
