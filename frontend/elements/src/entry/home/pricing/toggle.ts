import { LitElement, html, css, customElement, property } from "lit-element";

type Mode = 'monthly' | 'annually';

@customElement("pricing-toggle")
export class _ extends LitElement {
    static get styles() {
        return [css`
            :host {
                display: flex;
                justify-content: center;
                gap: 10px;
                margin: 20px 0;
            }
            :host > * {
                cursor: pointer;
            }
            .toggle {
                display: grid;
                align-items: center;
            }
            .track {
                grid-column: 1;
                grid-row: 1;
                width: 40px;
                height: 12px;
                border-radius: 6px;
                background-color: var(--light-gray-2);
            }
            .dot {
                grid-column: 1;
                grid-row: 1;
                height: 20px;
                width: 20px;
                border-radius: 50%;
                background-color: var(--main-blue);
                transition: translate .3s;
            }
            :host([value=monthly]) .dot {
                translate: 0%;
            }
            :host([value=annually]) .dot {
                translate: 100%;
            }
            .label {
                font-size: 14px;
                color: var(--dark-gray-6);
                background-color: #52525210;
                padding: 2px 6px;
                border-radius: 4px;
            }
            :host([value=monthly]) .label.monthly,
            :host([value=annually]) .label.annually {
                background-color: var(--yellow-4);
            }
            .annual-tag {
                font-size: 13px;
                font-weight: 600;
                padding: 0 8px;
                border-radius: 4px;
            }
        `];
    }

    @property({ reflect: true })
    value: Mode = "monthly";

    @property()
    annual_label: string = "";

    toggle() {
        const value = this.value === "annually" ? "monthly" : "annually";
        this.change(value);
    }

    change(value: Mode) {
        this.dispatchEvent(new CustomEvent("custom-string", {
            detail: {
                value,
            }
        }));
    }

    render() {
        return html`
            <div class="label monthly" @click=${() => this.change("monthly")}>Monthly</div>
            <div class="toggle" @click=${this.toggle}>
                <div class="track"></div>
                <div class="dot"></div>
            </div>
            <div class="label annually" @click=${() => this.change("annually")}>
                Annual
                <span class="annual-tag">${this.annual_label}</span>
            </div>
        `;
    }
}
