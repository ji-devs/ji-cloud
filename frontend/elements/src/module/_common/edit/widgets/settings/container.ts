import {
    LitElement,
    html,
    css,
    customElement,
} from "lit-element";
import "@elements/core/images/ui";

@customElement("module-settings-container")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
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
