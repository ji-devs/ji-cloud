import { LitElement, html, css, customElement, property } from 'lit-element';
import { ifDefined } from 'lit-html/directives/if-defined';


@customElement('step-nav')
export class _ extends LitElement {

    static get styles() {
        return [css`
            :host {
                display: contents;
            }
            :host(:last-child) .line {
                display: none;
            }
            .line {
                width: 100%;
                margin-top: 25px;
                display: grid;
            }
            .line::after {
                content: '';
                display: inline-block;
                background-color: var(--light-gray-1);
                height: 2px;
                /* 50px matches the width of the circle-button */
                width: calc(100% + 50px);
            }
            :host([completed]) .line::after {
                background-color: var(--dark-green-1);
            }
        `];
    }

    @property({type: Number})
    number: number = 1;

    @property({type: String})
    label: string = "";

    @property({type: Boolean, reflect: true})
    completed: boolean = false;

    @property({type: Boolean})
    active: boolean = false;

    render() {
        return html`
            <button-circle
                label="${this.label}"
                color="${ifDefined(
                    this.active ?  "blue"
                        : this.completed ? "green"
                        : undefined
                )}"
            >
                ${this.number}
            </button-circle>
            <div class="line"></div>
        `;
    }
}
