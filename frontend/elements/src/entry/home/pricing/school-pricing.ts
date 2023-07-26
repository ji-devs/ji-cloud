import { LitElement, html, css, customElement, property } from "lit-element";
import { percentage, price } from "./table";
import { nothing } from "lit-html";

@customElement("pricing-school-pricing")
export class _ extends LitElement {
    static get styles() {
        return [css`
            :host {
                display: grid;
                justify-items: center;
                padding-block: 22px;
            }
            h3 {
                font-size: 14px;
                font-weight: 600;
                color: var(--dark-blue-4);
                margin: 0;
            }
            .options-wrapper {
                height: 48px;
                width: 300px;
                display: grid;
                padding: 4px 5px;
                border: solid 1px var(--light-blue-6);
                border-radius: 48px;
                justify-content: center;
                margin: 8px 0;
                margin-top: 28px;
                margin-bottom: 8px;
                box-sizing: border-box;
            }
            .indicator-wrapper {
                grid-column: 1;
                grid-row: 1;
                z-index: 1;
            }
            .indicator {
                width: 58px;
                height: 100%;
                background-color: var(--main-blue);
                border-radius: 48px;
                transition: transform .3s;
            }
            .options {
                grid-column: 1;
                grid-row: 1;
                z-index: 2;
                font-size: 12px;
                text-align: center;
                color: var(--dark-gray-3);
                display: flex;
            }
            .options label {
                width: 58px;
                display: grid;
                align-content: center;
                cursor: pointer;
                line-height: 1.6em;
                position: relative;
            }
            .options .label-top {
                position: absolute;
                margin-top: -26px;
                margin-left: -2px;
                width: 62px;
            }
            .options .count {
                font-size: 18px;
                font-weight: 700;
                transition: color .3s;
            }
            .options input {
                display: none;
            }
            .price-line {
                display: grid;
                grid-template-columns: 1fr auto 1fr;
                gap: 8px;
            }
            .price {
                grid-column: 2;
                font-size: 38px;
                font-weight: 700;
                color: var(--dark-gray-6);
                line-height: 1;
            }
            .price-original {
                grid-column: 3;
                font-size: 16px;
                font-weight: 600;
                color: var(--dark-gray-3);
                text-decoration: line-through;
            }
            .annually {
                font-size: 13px;
                color: var(--dark-gray-6);
                margin-bottom: 12px;
            }
        `];
    }

    @property({ type: Number, reflect: true })
    selectedIndex: number = 1;

    @property({ type: Number, reflect: true })
    plan_price?: number;

    @property({ type: Number, reflect: true })
    discount_percentage?: number;

    private onChange(index: number) {
        this.dispatchEvent(new CustomEvent("custom-number", {
            detail: {
                number: index,
            }
        }));
    }

    render() {
        return html`
            <style>
                .indicator {
                    transform: translateX(${this.selectedIndex}00%);
                }
                .options label:nth-child(${this.selectedIndex + 1}) .count {
                    color: #fff;
                }
            </style>
            <h3>How many pro-accounts?</h3>
            <div class="options-wrapper">
                <div class="indicator-wrapper">
                    <div class="indicator"></div>
                </div>
                <div class="options">
                    <label>
                        <span class="label-top">up to</span>
                        <span class="count">4</span>
                        <input name="count" type="radio" @change=${() => this.onChange(0)}>
                    </label>
                    <label>
                        <span class="label-top">up to</span>
                        <span class="count">10</span>
                        <input name="count" type="radio" @change=${() => this.onChange(1)}>
                    </label>
                    <label>
                        <span class="label-top">up to</span>
                        <span class="count">20</span>
                        <input name="count" type="radio" @change=${() => this.onChange(2)}>
                    </label>
                    <label>
                        <span class="label-top">up to</span>
                        <span class="count">30</span>
                        <input name="count" type="radio" @change=${() => this.onChange(3)}>
                    </label>
                    <label>
                        <span class="label-top">More than</span>
                        <span class="count">30+</span>
                        <input name="count" type="radio" @change=${() => this.onChange(4)}>
                    </label>
                </div>
            </div>
            <div class="price-line">
                <div class="price">${price(
                    this.discount_percentage ? percentage(this.plan_price, this.discount_percentage) : this.plan_price
                )}</div>
                <div class="price-original">
                    ${this.discount_percentage ? price(this.plan_price) : nothing }
                </div>
            </div>
            <div class="annually">Annually</div>
            <slot name="start-button"></slot>
        `;
    }
}
