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
                font-size: 14px;
                font-weight: 500;
                color: var(--dark-gray-6);
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
            .annual-tag {
                background-color: var(--yellow-4);
                font-size: 13px;
                font-weight: 600;
                padding: 0 8px;
                border-radius: 4px;
                margin-left: 8px;
            }
        `];
    }

    @property({ reflect: true })
    value: Mode = "monthly";

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
            <!-- <div @click=${() => this.change("monthly")}>Monthly</div>
            <div class="toggle" @click=${this.toggle}>
                <div class="track"></div>
                <div class="dot"></div>
            </div>
            <div @click=${() => this.change("annually")}>
                Annual
                <span class="annual-tag">Get 2 months FREE!</span>
            </div> -->
        `;
    }
}
