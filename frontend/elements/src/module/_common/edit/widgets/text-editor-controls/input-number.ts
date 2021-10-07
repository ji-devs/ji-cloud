import { LitElement, html, css, customElement, property } from "lit-element";


@customElement("text-editor-controls-input-number")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: grid;
                    grid-template-columns: 36px 1fr 36px;
                    border: solid 1px var(--light-blue-5);
                    border-radius: 14px;
                    justify-content: space-evenly;
                    padding: 14px 1px;
                }
                @media (min-width: 1920px) {
                    :host {
                        padding: 14px 24px;
                        column-gap: 24px;
                    }
                }
                :host(:focus-within) {
                    border: solid 2px var(--dark-blue-3);
                }
                :host(:focus-within) {
                    /* removing one pixel to account for thicker border */
                    padding: 13px 0;
                }
                @media (min-width: 1920px) {
                    :host(:focus-within) {
                        /* removing one pixel to account for thicker border */
                        padding: 13px 23px;
                    }
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
                label {
                    grid-column: 2;
                    grid-row: 1;
                    display: grid;
                    place-content: center;
                }
                select {
                    border: 0;
                    padding: 0;
                    text-align: center;
                    font-family: Poppins;
                    font-size: 18px;
                    font-weight: normal;
                    font-stretch: normal;
                    font-style: normal;
                    width: 100%;
                    flex-grow: 1;
                    background-color: transparent;
                    appearance: none;
                    text-align-last: center;
                    opacity: 0;
                    grid-column: 2;
                    grid-row: 1;
                }
                select:focus {
                    outline: 0;
                }
            `,
        ];
    }

    @property({type: Number})
    value: number = 1;

    private min: number = 0;

    private max: number = 300;

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

    private onSelectChange = (e: any) => {
        this.value = parseInt(e.target.value);
        this.changed();
    }

    private getSelectOptions(): number [] {
        let arr =  Array.from(Array(17).keys());
        arr.shift();
        return arr;
    }

    render() {
        return html`
            <button @click="${this.decrement}">-</button>
            <label for="select">${ this.value.toString() }</label>
            <select id="select" @input=${this.onSelectChange}>
                ${
                    this.getSelectOptions()
                        .map((num: number) => html`
                            <option>${num * 10}</option>
                        `)
                }
            </select>
            <button @click="${this.increment}">+</button>
        `;
    }
}
