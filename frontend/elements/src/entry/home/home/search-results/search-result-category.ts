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
                    color: var(--white);
                }

                :host([filled]) {
                    border: solid 1px var(--light-orange-3);
                    background-color: var(--white);
                    color: var(--dark-gray-5);
                }
            `,
        ];
    }

    @property()
    label: string = "";

    @property({ type: Boolean, reflect: true })
    filled: boolean = false;

    render() {
        return html` ${this.label} `;
    }
}
