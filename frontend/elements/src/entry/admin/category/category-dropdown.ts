import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/titles/ji";
import "@elements/core/titles/variants/underlined-title";
import "@elements/core/inputs/composed/search";
@customElement("category-dropdown")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                li {
                    cursor: pointer;
                }
                ul {
                    list-style-type: none;

                    display: block;
                    z-index: 2;
                    background-color: #ffffff;
                    position: absolute;
                    top: 0;
                    left: 85px;
                    width: 125px;
                    border-radius: 4px;
                    box-shadow: 0 3px 16px 0 rgba(0, 0, 0, 0.25);
                    padding: 16px;
                }
            `,
        ];
    }

    render() {
        const STR_SIBLING = "Add a sibling";
        const STR_CHILD = "Add a child";
        const STR_RENAME = "Rename";
        const STR_DELETE = "Delete";
        const STR_HIDE = "Hide";

        return html`
            <ul>
                <li>${STR_SIBLING}</li>
                <li>${STR_CHILD}</li>
                <li>${STR_RENAME}</li>
                <li>${STR_DELETE}</li>
                <li>${STR_HIDE}</li>
            </ul>
        `;
    }
}
