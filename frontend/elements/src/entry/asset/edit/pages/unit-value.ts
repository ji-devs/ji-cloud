import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/images/ui";

@customElement("unit-edit-value-file")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: grid;
                    grid-auto-columns: minmax(0px, 1fr);
                    justify-content: space-between;
                    column-gap: 8px;
                    row-gap: 12px;
                    background-color: var(--light-blue-1);
                    padding: 1.5em;
                    border-radius: 8px;
                }
                .label {
                    overflow: hidden;
                    white-space: nowrap;
                    text-overflow: ellipsis;
                }
                .label fa-icon {
                    font-size: 12px;
                    border-radius: 50%;
                    background-color: var(--dark-green-1);
                    color: #fff;
                    height: 24px;
                    width: 24px;
                    display: inline-grid;
                    place-content: center;
                }
                ::slotted([slot=delete]) {
                    color: var(--main-blue);
                }
            `,
        ];
    }

    @property()
    label: string = "";

    @property()
    resourceHref: string = "";

    render() {
        return html`
            <div class="label" title=${this.label}>
                <fa-icon icon="fa-light fa-check"></fa-icon>
                ${this.resourceHref === "" 
                    ? html`${ this.label }` 
                    : html`<a href="${this.resourceHref}" target="_blank">${ this.label }</a>` }
            </div>
        `;
    }
}
