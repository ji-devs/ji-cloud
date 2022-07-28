import { LitElement, html, css, customElement, property } from "lit-element";

@customElement("home-search-result-category")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: inline-block;
                    box-sizing: border-box;
                    padding: 2px 12px;
                    font-size: 14px;
                    border-radius: 16px;
                    border: solid 1px var(--white);
                    color: var(--dark-gray-5);
                    background-color: var(--white);
                }
            `,
        ];
    }

    @property()
    label: string = "";

    render() {
        return html` ${this.label} `;
    }
}
