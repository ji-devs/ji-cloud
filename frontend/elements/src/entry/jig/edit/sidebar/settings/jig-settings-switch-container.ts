import { LitElement, html, css, customElement, property } from "lit-element";

@customElement("jig-settings-switch-container")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                label {
                    display: flex;
                    column-gap: 12px;
                    align-items: center;
                }
            `,
        ];
    }

    render() {
        return html`
            <label>
                <slot name="switch"></slot>
                <slot name="label"></slot>
            </label>
        `;
    }
}
