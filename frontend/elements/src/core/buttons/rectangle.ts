import { LitElement, html, css, customElement, property } from "lit-element";
import { ifDefined } from "lit-html/directives/if-defined";

export type Color = "red" | "blue" | "green" | "grey";
export type Kind = "filled" | "text" | "outline";
export type Size = "regular" | "large";

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
                    cursor: pointer;
                    display: inline-grid;
                    place-content: center;
                }

                :host([color="red"]) {
                    --color: var(--main-red);
                }
                :host([color="red"]:hover) {
                    --color: var(--dark-red-1);
                }
                :host([color="blue"]) {
                    --color: var(--dark-blue-1);
                }
                :host([color="blue"]:hover) {
                    --color: var(--dark-blue-3);
                }
                :host([color="green"]) {
                    --color: var(--main-green);
                }
                :host([color="green"]:hover) {
                    --color: var(--dark-green-1);
                }
                :host([color="grey"]) {
                    --color: var(--dark-gray-3);
                }
                :host([color="grey"]:hover) {
                    --color: var(--dark-gray-4); /*my own color*/
                }

                :host([disabled]) {
                    pointer-events: none;
                    cursor: not-allowed;
                    --color: var(--light-gray-4);
                }

                :host([kind="filled"]) {
                    background-color: var(--color);
                    color: #ffffff;
                }
                :host([kind="outline"]) {
                    color: var(--color);
                    border: solid 1px var(--color);
                }
                :host([kind="text"]) {
                    color: var(--color);
                    border-radius: 0;
                }

                :host([size="regular"]) {
                    font-size: 14px;
                    border-radius: 16px;
                }
                :host([size="large"]) {
                    font-size: 16px;
                    border-radius: 20px;
                }

                :host([size="regular"]) .inner {
                    height: 32px;
                }
                :host([size="large"]) .inner {
                    height: 40px;
                }
                :host([size="regular"][kind="filled"]) .inner,
                :host([size="regular"][kind="outline"]) .inner {
                    padding-inline: 16px;
                }
                :host([size="large"][kind="filled"]) .inner,
                :host([size="large"][kind="outline"]) .inner {
                    padding-inline: 24px;
                }

                .inner {
                    all: unset;
                    user-select: none;
                    color: inherit;
                    display: flex;
                    align-items: center;
                    column-gap: 5px;
                    font-weight: 600;
                }

                /* button.inner {
                    display: flex;
                    flex-direction: row;
                    align-items: center;
                } */
            `,
        ];
    }

    @property({ reflect: true })
    size: Size = "regular";

    @property({ reflect: true })
    color: Color = "red";

    @property({ reflect: true })
    kind: Kind = "filled";

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
        if (this.disabled) e.stopPropagation();
    }

    private renderButton() {
        return html`
            <button
                part="button"
                class="inner"
                type="${this.submit ? "submit" : "button"}"
                ?disabled="${this.disabled}"
            >
                <slot></slot>
            </button>
        `;
    }
    private renderLink() {
        return html`
            <a
                part="a"
                class="inner"
                href=${this.href!}
                @click=${this.onClick}
                target=${ifDefined(this.target as any)}
            >
                <slot></slot>
            </a>
        `;
    }

    render() {
        return this.href === undefined
            ? this.renderButton()
            : this.renderLink();
    }
}
