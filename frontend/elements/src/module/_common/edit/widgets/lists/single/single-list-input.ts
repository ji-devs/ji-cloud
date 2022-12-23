import { LitElement, html, css, customElement, property } from "lit-element";
import { live } from "lit-html/directives/live";

@customElement("sidebar-widget-single-list-input")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: flex;
                    justify-content: center;
                }
                input {
                    height: 30px;
                    width: calc(100% - 32px); /*to not go into rounded corners*/
                    outline: none;
                    border: none;
                    font-size: 18px;
                    text-align: center;
                }

                :host([placeholder]) input {
                    color: var(--light-gray-4);
                }

                :host([h]) {
                    display: none;
                }
            `,
        ];
    }

    @property()
    constrain: ((text: string) => string) | undefined = undefined;

    setValue = (value: string) => {
        const { constrain } = this;
        this.value = constrain ? constrain(value) : value;
    };
    onInput(evt: InputEvent) {
        this.setValue((evt.target as any).value);
        this.dispatchEvent(
            new CustomEvent("custom-input", {
                detail: { value: this.value },
            })
        );
    }
    onChange(evt: InputEvent) {
        this.setValue((evt.target as any).value);
        this.dispatchEvent(
            new CustomEvent("custom-change", {
                detail: { value: this.value },
            })
        );
    }

    @property({ hasChanged: () => true })
    value: string = "";

    @property({ type: Boolean, reflect: true })
    placeholder: boolean = false;

    @property({ type: Boolean, reflect: true })
    h: boolean = false;

    render() {
        const { value } = this;

        return html`
            <input
                type="text"
                @input="${this.onInput}"
                @change="${this.onChange}"
                .value="${live(value)}"
            />
        `;
    }
}
