import { LitElement, html, css, customElement, property } from "lit-element";
import { listItemStyles } from "./list-item-styles";
import { nothing } from "lit-html";

@customElement("community-list-member")
export class _ extends LitElement {
    static get styles() {
        return [
            listItemStyles,
            css`
                p {
                    margin: 0;
                }
                :host {
                    cursor: pointer;
                    min-height: 66px;
                    padding-top: 10px;
                    padding-bottom: 10px;
                    border-radius: 14px;
                    box-shadow: 0 0 6px 0 rgba(0, 0, 0, 0.08);
                    border: solid 1px var(--light-orange-3);
                    background-color: #ffffff;
                    font-size: 14px;
                    font-weight: 500;
                    color: var(--dark-gray-6);
                }
                ::slotted([slot=img]) {
                    height: 50px;
                    width: 50px;
                    border-radius: 50%;
                }
                .location {
                    display: inline-grid;
                    grid-template-columns: auto auto;
                    gap: 12px;
                }
                .location img-ui {
                    width: 37px;
                }
            `,
        ];
    }

    @property()
    name: string = "";

    @property()
    countryCode: string = "";

    @property()
    countryName: string = "";

    @property()
    language: string = "";

    render() {
        return html`
            <slot name="img"></slot>
            <p class="name">${this.name}</p>
            <p class="desktop-only location" title=${this.countryName}>
                ${this.countryCode ? html`
                    <span>${this.countryCode}</span>
                    <img-ui path="flags/${this.countryCode}.webp"></img-ui>
                ` : nothing}
            </p>
            <p class="desktop-only language">${this.language}</p>
            <slot class="desktop-only status" name="status"></slot>
        `;
    }
}
