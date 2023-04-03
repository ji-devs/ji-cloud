import { LitElement, html, css, customElement, property, internalProperty, PropertyValues } from "lit-element";

const MINUTE = 60;

@customElement("input-minutes-seconds")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: inline-grid;
                    grid-template-columns: minmax(0, 1fr) min-content minmax(0, 1fr);
                }
                input {
                    padding: 0;
                    border: 0;
                    background-color: transparent;
                    font: inherit;
                }
                input:focus {
                    outline: 0;
                }
                input#minutes {
                    text-align: right;
                }
                input#seconds {
                    text-align: left;
                }
                input::-webkit-outer-spin-button,
                input::-webkit-inner-spin-button {
                    appearance: none;
                }
            `,
        ];
    }

    @property({ type: Number })
    value?: number;

    @internalProperty()
    minutes?: number;

    @internalProperty()
    seconds?: number;
  
    // firstUpdated() {
    //     this.tabIndex = 0;
    //     this.setAttribute("tabindex", "0");
    // }

    updated(propertyValues: PropertyValues) {
        if (propertyValues.has("value")) {
            console.log("updated");
            
            this.programingUpdate();
        }
    }

    private onMinutesChange(e: Event) {
        const valueAsNumber = (e.target as HTMLInputElement).valueAsNumber;
        console.log((e.target as HTMLInputElement).valueAsNumber);
        
        if(window.isNaN(valueAsNumber))
            this.minutes = undefined;
        else
            this.minutes = valueAsNumber;
        
        console.log(this.minutes);

        this.userUpdate();
    }

    private onSecondsChange(e: Event) {
        const valueAsNumber = (e.target as HTMLInputElement).valueAsNumber;
        if(window.isNaN(valueAsNumber))
            this.seconds = undefined;
        else
            this.seconds = valueAsNumber;
        this.userUpdate();
    }

    private userUpdate() {
        console.log(this.minutes, this.seconds);
        let value;
        if((this.minutes || this.minutes === 0) && (this.seconds || this.seconds === 0)) {
            // this.value = (this.minutes * MINUTE) + this.seconds;
            value = (this.minutes * MINUTE) + this.seconds;
            console.log(true);
            
        } else {
            value = null;
            // console.log(false);

            // this.value = undefined;
        }
        // this.dispatchEvent(new Event("input"));
        this.dispatchEvent(
            new CustomEvent("custom-input-number", {
                detail: { value },
            })
        );
        
    }

    private programingUpdate() {
        if(!this.value && this.value !== 0) {
            this.seconds = undefined;
            this.minutes = undefined;
        } else {
            this.seconds = this.value % MINUTE;
            this.minutes = Math.floor(this.value / MINUTE);
        }
    }

    render() {
        return html`
            <input
                id="minutes"
                type="number"
                @input=${this.onMinutesChange}
                value=${this.minutes ?? ""}
                min="0"
                placeholder="MM"
            >
            :
            <input
                id="seconds"
                type="number"
                @input=${this.onSecondsChange}
                value=${paddingNumber(this.seconds)}
                max="59"
                min="0"
                placeholder="SS"
            >
        `;
    }

    // createRenderRoot() {
    //     return this.attachShadow({ mode: "open", delegatesFocus: true });
    // }
}

function paddingNumber(number: number | undefined): string {
    if(!number && number !== 0) {
        return "";
    } else {
        return number.toString().padStart(2, "0");
    }
}
