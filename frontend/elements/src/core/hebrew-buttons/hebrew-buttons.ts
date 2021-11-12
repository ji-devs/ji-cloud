import { LitElement, html, css, customElement, property } from "lit-element";
import "./hebrew-keyboard/hebrew-keyboard";

export const KEYBOARD_HEIGHT = 216;

@customElement("hebrew-buttons")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: inline-flex;
                    grid-gap: 12px;
                    align-items: center;
                }
                :host(:not([full]):not(:hover)) ::slotted([slot="full-only"]) {
                    display: none;
                }
                ::slotted(.divider) {
                    width: 1px;
                    height: 20px;
                    background-color: var(--main-blue);
                }
            `,
        ];
    }

    @property({ type: Boolean, reflect: true })
    full: boolean = false;

    render() {
        return html`
            <slot name="full-only"></slot>
            <slot name="always"></slot>
        `;
    }
}
