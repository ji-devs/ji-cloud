import {
    LitElement,
    html,
    svg,
    css,
    customElement,
    property,
} from "lit-element";
import { nothing } from "lit-html";
import { classMap } from "lit-html/directives/class-map";
import "@elements/core/images/ui";

@customElement("module-settings-container")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                }

                ::slotted(:not(:first-child)) {
                    padding-top: 24px;
                    border-top: solid 1px var(--light-blue-4);
                }
                ::slotted(:not(:last-child)) {
                    padding-bottom: 24px;
                }
            `,
        ];
    }

    render() {
        return html` <slot></slot> `;
    }
}
