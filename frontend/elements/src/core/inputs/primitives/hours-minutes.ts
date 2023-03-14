import { LitElement, html, css, customElement, property, internalProperty, PropertyValues } from "lit-element";

const HOUR = 24;

@customElement("input-hours-minutes")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: inline-grid;
                    grid-template-columns: minmax(0, 20px) min-content minmax(0, 30px) min-content;
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
                input#hours {
                    text-align: right;
                }
                input#minutes {
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
    hours?: number;

    @internalProperty()
    minutes?: number;
  
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

    private onHoursChange(e: Event) {
        const valueAsNumber = (e.target as HTMLInputElement).valueAsNumber;
        console.log((e.target as HTMLInputElement).valueAsNumber);
        
        if(window.isNaN(valueAsNumber))
            this.hours = undefined;
        else
            this.hours = valueAsNumber;
        
        console.log(this.hours);

        this.userUpdate();
    }

    private onMinutesChange(e: Event) {
        const valueAsNumber = (e.target as HTMLInputElement).valueAsNumber;
        if(window.isNaN(valueAsNumber))
            this.minutes = undefined;
        else
            this.minutes = valueAsNumber;
        this.userUpdate();
    }

    private userUpdate() {
        console.log(this.hours, this.minutes);
        let value;
        if((this.hours || this.hours === 0) && (this.minutes || this.minutes === 0)) {
            // this.value = (this.hours * HOUR) + this.seconds;
            value = (this.hours * HOUR) + this.minutes;
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
            this.hours = undefined;
            this.minutes = undefined;
        } else {
            this.minutes = this.value % HOUR;
            this.hours = Math.floor(this.value / HOUR);
        }
    }

    render() {
        return html`
            <input
                id="hours"
                type="number"
                @input=${this.onHoursChange}
                value=${this.hours ?? ""}
                min="0"
                placeholder="HH"
            >
            :
            <input
                id="minutes"
                type="number"
                @input=${this.onMinutesChange}
                value=${paddingNumber(this.minutes)}
                max="59"
                min="0"
                placeholder="MM"
            > H
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
