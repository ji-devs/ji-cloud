import { LitElement, html, css, customElement, property } from "lit-element";

@customElement("pricing-banner")
export class _ extends LitElement {
    static get styles() {
        return [css`
            :host {
                background-color: var(--light-blue-6);
                display: grid;
                padding: 0 20px;
            }
            h1 {
                font-size: 28px;
                font-weight: 900;
                text-align: center;
                color: var(--main-yellow);
                margin: 24px 0;
            }
            .tabs {
                display: grid;
                grid-template-columns: repeat(2, minmax(160px, 300px));
                justify-content: center;
                text-align: center;
                height: 40px;
            }
            .tabs ::slotted([slot=tab]) {
                font-size: 18px;
                font-weight: 600;
                display: grid;
                place-content: center;
                border-top-left-radius: 20px;
                border-top-right-radius: 20px;
                cursor: pointer;
                background-color: var(--main-blue);
                color: var(--light-blue-3);
                border: 0;
            }
            :host([tab=individual]) .tabs ::slotted([slot=tab].active) {
                background-color: #ffffff;
                color: var(--main-blue);
            }
        `];
    }

    @property({ reflect: true })
    tab: 'individual' | 'school' = "individual";

    render() {
        return html`
            <h1>Jigzi plans</h1>
            <div class="tabs">
                <slot name="individual"></slot>
                <slot name="school"></slot>
                <slot name="tab"></slot>
            </div>
        `;
    }
}
