import { LitElement, html, css, customElement } from "lit-element";

@customElement("module-card-print-list")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    width: 100%;
                    display: block;
                }
                .cards {
                    --border: dashed 1px #a1a8ad;
                    text-align: center;
                }
            `,
        ];
    }

    render() {
        return html`
            <div class="cards">
                <slot></slot>
            </div>
        `;
    }
}
