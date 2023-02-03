import { LitElement, html, css, customElement, property } from "lit-element";
import { ifDefined } from "lit-html/directives/if-defined";

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

@customElement("hover-circle")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    border: none;
                    cursor: pointer;
                    display: inline-grid;
                    place-content: center;
                    padding: 0;
                    user-select: none;
                    line-height: 30px;
                    font-size: 20px;
                }

                button,
                a {
                    all: unset;
                    display: flex;
                    column-gap: 10px;
                }

                a:hover {
                    border-radius: 100%;
                    background-color: #c4d9f7;
                }

                button {
                    display: flex;
                    flex-direction: row;
                    align-items: center;
                }
            `,
        ];
    }

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
