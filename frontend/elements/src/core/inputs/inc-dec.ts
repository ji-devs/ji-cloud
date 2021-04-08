import { LitElement, html, css, customElement, property } from "lit-element";


@customElement("input-inc-dec")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: flex;
                    border: solid 1px var(--light-blue-5);
                    border-radius: 14px;
                    justify-content: space-evenly;
                    padding: 14px 24px;
                    column-gap: 24px;
                }
                :host(:focus-within) {
                    border: solid 2px var(--dark-blue-3);
                    /* removing one pixel to account for thicker border */
                    padding: 13px 23px;
                }
                button {
                    padding: 0px;
                    background-color: transparent;
                    border: 0px;
                    font-size: 26px;
                    font-weight: 100;
                    line-height: 1em;
                    height: 36px;
                    width: 36px;
                    color: var(--light-blue-5);
                    cursor: pointer;
                }
                input {
                    -moz-appearance: textfield;
                    border: 0;
                    padding: 0;
                    text-align: center;
                    font-family: Poppins;
                    font-size: 18px;
                    font-weight: normal;
                    font-stretch: normal;
                    font-style: normal;
                    width: 30px;
                    flex-grow: 1;
                }
                input::-webkit-outer-spin-button, input::-webkit-inner-spin-button {
                    display: none;
                }
                input:focus {
                    outline: 0;
                }
            `,
        ];
    }

    @property({type: Number})
    value: number = 1;

    @property({type: Number})
    min: number = 0;

    @property({type: Number})
    max: number = 10;

    private changed() {
        this.dispatchEvent(new CustomEvent("custom-change", {
            detail: { value: this.value.toString() }, // custom change expect a string
        }))
    }

    private increment() {
        if (this.value < this.max) {
            this.value++;
            this.changed();
        }
    }

    private decrement() {
        if (this.value > this.min) {
            this.value--;
            this.changed();
        }
    }

    private onInput = (e: InputEvent) => {
        let newValue = (e.target as HTMLInputElement).valueAsNumber;
        if(!Number.isNaN(newValue)) {
            if (newValue > this.max) // too high
                this.value = this.max;
            else if (newValue < this.min) // to low
                this.value = this.min;
            else
                this.value = newValue;

            this.changed();
        }
    }

    render() {
        return html`
            <button @click="${this.decrement}">-</button>
            <input type="number" @input="${this.onInput}" .value="${ this.value.toString() }">
            <button @click="${this.increment}">+</button>
        `;
    }
}
