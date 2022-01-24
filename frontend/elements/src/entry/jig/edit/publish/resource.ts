import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/images/ui";

@customElement("jig-edit-publish-resource")
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
                    background-color: var(--green-2);
                    padding: 16px;
                    border-radius: 12px;
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
                .second-line {
                    display: flex;
                    justify-content: space-between;
                }
                .resource-type {
                    display: inline-block;
                    font-size: 13px;
                    color: var(--dark-blue-1);
                    padding: 3px 12px;
                    background-color: #ffffff;
                    border-radius: 16px;
                    border: solid 1px #b0ccf2;
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
    resourceType: string = "";

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
            <div class="second-line">
                <span class="resource-type">${this.resourceType}</span>
                <slot name="delete"></slot>
            </div>
        `;
    }
}
