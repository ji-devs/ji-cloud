import { LitElement, html, css, customElement, property } from "lit-element";

@customElement("home-search-result-category")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: inline-block;
                    box-sizing: border-box;
                    border: solid 1px var(--light-blue-4);
                    padding: 3px 12px;
                    font-size: 14px;
                    border-radius: 20px;
                    margin: 0 8px 8px 0;
                    color: var(--light-blue-4);
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
