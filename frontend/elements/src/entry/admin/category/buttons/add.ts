import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/buttons/rectangle-icon";

@customElement("category-button-add")
export class _ extends LitElement {
    render() {
        const STR_ADD = "Add Category";
        return html`<button-rect-icon color="blue" iconBefore="plus"
            >${STR_ADD}</button-rect-icon
        >`;
    }
}
