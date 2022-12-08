import { LitElement, html, css, customElement, property } from "lit-element";

@customElement("color-select-section")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: grid;
                    row-gap: 16px;
                }
                h4 {
                    margin: 0;
                    font-weight: 600;
                    color: #4a4a4a;
                    font-size: 15px;
                }
                .items {
                    display: grid;
                    gap: 16px;
                    grid-template-columns: repeat(5, 54px);
                }
            `,
        ];
    }

    @property()
    label: string = "";

    render() {
        return html`
            <h4>${this.label}</h4>
            <div class="items">
                <slot name="items"></slot>
            </div>
        `;
    }
}
