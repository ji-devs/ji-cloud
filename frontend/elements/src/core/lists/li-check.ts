import { LitElement, html, css, customElement, property } from "lit-element";
import { nothing } from "lit-html";

@customElement("li-check")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                li {
                    margin-bottom: 4px;
                    list-style-type: none;
                    display: flex;
                    justify-content: space-between;
                }
                li:hover {
                    background-color: #e7f0fe;
                }
                p {
                    padding-left: 20px;
                    padding-right: 20px;
                    margin: 0;
                }
                img-ui {
                    display: block;
                    padding-right: 20px;
                }
            `,
        ];
    }

    @property({ type: Boolean })
    selected: boolean = false;

    render() {
        const { selected } = this;

        return html`
            <li>
                <p><slot></slot></p>
                ${selected
                    ? html`<img-ui
                          class="${selected ? "checked" : ""}"
                          path="core/lists/check.svg"
                      ></img-ui>`
                    : nothing}
            </li>
        `;
    }
}
