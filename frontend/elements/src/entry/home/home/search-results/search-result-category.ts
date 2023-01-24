import { LitElement, html, css, customElement, property } from "lit-element";

@customElement("home-search-result-category")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: inline-block;
                    box-sizing: border-box;
                    padding: 2px 8px;
                    font-size: 13px;
                    border-radius: 16px;
                    border: solid 1px var(--line-color);
                    color: var(--dark-gray-5);
                    background-color: var(--white);
                    margin: 6px 0;
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
