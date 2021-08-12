import { LitElement, html, css, customElement, queryAll, property } from 'lit-element';

const STR_CLEAR = "Clear";

@customElement('home-student-code-input')
export class _ extends LitElement {
    static get styles() {
        return [css`
            :host {
                display: grid;
                row-gap: 80px
            }
            .inputs {
                display: grid;
                grid-template-columns: repeat(4, 80px);
                column-gap: 40px;
                justify-content: center;
            }
            .inputs input {
                font-size: 40px;
                border: solid 1px var(--main-blue);
                border-radius: 14px;
                font-weight: bold;
                text-align: center;
                color: var(--light-blue-5);
                box-sizing: border-box;
                height: 80px;
            }
            .inputs input:focus {
                outline: 0;
                border: solid 2px #1160fb;
            }
            :host([error]) .inputs input {
                border: solid 1px #f00b19;
                background-color: var(--light-red-alert);
                color: var(--dark-red-1);
            }
            :host([error]) .inputs input:focus {
                outline: 0;
                border: solid 2px #f00b19;
            }
            input[type=number] {
                -moz-appearance: textfield;
            }
            input::-webkit-outer-spin-button, input::-webkit-inner-spin-button {
                -webkit-appearance: none;
            }

            .backspace {
                justify-self: end;
                background-color: transparent;
                width: 100px;
                height: 48px;
                border: solid 3px var(--main-blue);
                border-left: 0;
                border-radius: 14px 12px 12px 14px;
                color: var(--main-blue);
                font-size: 20px;
                font-weight: 600;
                cursor: pointer;
                display: flex;
                align-items: center;
                justify-content: space-evenly;
            }
            .backspace:hover, .backspace:active {
                color: var(--dark-blue-2);
                border-color: var(--dark-blue-2);
            }
            .backspace::before {
                content: "";
                display: inline-block;
                width: 31px;
                height: 31px;
                transform: rotate(45deg) translate(-29px, 29px);
                border-left: solid 3px var(--main-blue);
                border-bottom: solid 3px var(--main-blue);
                border-radius: 5px 0 5px 8px;
                position: absolute;
            }
            .backspace:hover::before, .backspace:active::before {
                border-color: var(--dark-blue-2);
            }
            .backspace .icon {
                font-size: 30px;
            }
        `];
    }

    @property({ type: Boolean, reflect: true })
    error: boolean = false;

    @queryAll(".inputs input")
    private inputs!: NodeListOf<HTMLInputElement>;

    private isValidNumber(v: string): boolean {
        return /^\d$/.test(v);
    }

    private focusNextInput(current: HTMLInputElement) {
        for (let i = 0; i < this.inputs.length; i++) {
            if(this.inputs[i] === current) {
                if(i === this.inputs.length - 1) {
                    // last input
                    this.dispatchChangeEvent();
                } else {
                    this.inputs[i + 1].focus();
                    return;
                }
            }
        }
    }

    private dispatchChangeEvent() {
        let value = "";
        for (const input of this.inputs) {
            value += input.value;
        }
        this.dispatchEvent(new CustomEvent("custom-input", {
            detail: { value },
        }));
    }

    private onInput(e: InputEvent) {
        let target = (e.target as HTMLInputElement);
        if(this.isValidNumber(target.value)) {
            this.focusNextInput(target);
        } else {
            target.value = target.value.replace(/\D/g, "");
        }
    }

    private onPaste(e: ClipboardEvent) {
        let target = e.target as HTMLInputElement;
        let value = e.clipboardData!.getData("text");

        // remove all non digits
        value = value.replace(/\D/g, "");

        e.preventDefault();

        // if first input and is 4 digits
        if(value.length === 4 && this.inputs[0] === target) {
            for (let i = 0; i < this.inputs.length; i++) {
                this.inputs[i].value = value[i];
            }
            this.dispatchChangeEvent();
        } else if(value.length > 0) {
            target.value = value[0];
            this.focusNextInput(target);
        }
    }

    private clear() {
        for (const input of this.inputs) {
            input.value = "";
        }
        this.error = false;
    }

    render() {
        return html`
            <div class="inputs">
                <input @input=${this.onInput} @paste=${this.onPaste} type="number">
                <input @input=${this.onInput} @paste=${this.onPaste} type="number">
                <input @input=${this.onInput} @paste=${this.onPaste} type="number">
                <input @input=${this.onInput} @paste=${this.onPaste} type="number">
            </div>
            <button class="backspace" @click=${this.clear}>
                <span class="icon">&times;</span>
                ${STR_CLEAR}
            </button>
        `;
    }
}
