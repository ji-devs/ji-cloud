import { LitElement, html, css, customElement, property } from "lit-element";
import { nothing } from "lit-html";

@customElement("age-range")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    font-size: 13px;
                    display: flex;
                    align-items: center;
                    font-weight: 400;
                    column-gap: 4px;
                    color: var(--dark-gray-3);
                }
            `,
        ];
    }

    @property({ type: String })
    icon!: string;

    @property({ type: String })
    from?: string;

    @property({ type: String })
    to?: string;

    renderFrom() {
        return html`<span class="from">${this.from}</span>`
    }

    renderTo() {
        if (!this.to) {
            return nothing;
        }

        return html`
            <fa-icon icon="fa-thin fa-arrow-right"></fa-icon>
            <span class="to">${this.to}</span>
        `;
    }

    render() {
        if (!this.from) {
            // If the from age isn't set then don't render anything.
            return nothing;
        }

        return html`
            <img-ui path=${this.icon}></img-ui>
            ${this.renderFrom()}
            ${this.renderTo()}
        `;
    }
}

