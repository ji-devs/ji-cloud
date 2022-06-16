import { LitElement, html, css, customElement, property } from "lit-element";
import { ifDefined } from "lit-html/directives/if-defined";

@customElement("community-asset")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                a {
                    width: 232px;
                    height: 230px;
                    display: grid;
                    grid-template-columns: auto auto;
                    grid-template-rows: auto 1fr auto;
                    border-radius: 16px;
                    box-shadow: 0 3px 10px 0 rgba(0, 0, 0, 0.16);
                    background-color: #ffffff;
                    overflow: hidden;
                    cursor: pointer;
                }
                ::slotted([slot=thumbnail]) {
                    grid-column: 1 / -1;
                }
                .name {
                    font-size: 16px;
                    font-weight: 600;
                    text-align: center;
                    color: #555;
                    grid-column: 1 / -1;
                    margin: 0;
                }
                .ages {
                    font-size: 14px;
                    font-weight: 600;
                }
                .published-at {
                    font-size: 14px;
                    font-weight: 500;
                }
            `,
        ];
    }

    @property()
    name: string = "";

    @property()
    ages: string = "";

    @property()
    publishedAt: string = "";

    @property()
    href?: string;

    render() {
        return html`
            <a href=${ifDefined(this.href)} target="_BLANK">
                <slot name="thumbnail"></slot>
                <p class="name">${this.name}</p>
                <span class="ages">${this.ages}</span>
                <span class="published-at">${this.publishedAt}</span>
            </a>
        `;
    }
}
