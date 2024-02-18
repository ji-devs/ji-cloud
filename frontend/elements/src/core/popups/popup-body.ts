import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/images/ui";
import "@elements/core/progress-bar/progress-bar";

@customElement("popup-body")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: grid;
                    background-color: #fff;
                }
                header {
                    padding: 12px 12px 10px 12px;
                    min-height: 80px;
                }
                nav {
                    display: flex;
                    justify-content: space-between;
                }
                ::slotted([slot="back"]) {
                    /* only add margin in back exists */
                    margin-bottom: 12px;
                }
                ::slotted([slot="close"]) {
                    font-size: 20px;
                    color: var(--dark-gray-5);
                    font-weight: 300;
                    min-width: 30px;
                    min-width: 30px;
                    display: inline-grid;
                    place-content: center;
                }
                ::slotted(button-icon[slot="close"]) {
                    width: 12px;
                    height: 12px;
                }
                ::slotted([slot="heading"]) {
                    color: #fd7076;
                    font-size: 20px;
                    line-height: 1em;
                    font-weight: 600;
                    margin: 0 12px;
                }
                ::slotted([slot="author-line"]) {
                    font-size: 20px;
                    line-height: 1em;
                    font-weight: 600;
                    margin: 0 12px;
                }
            `,
        ];
    }

    render() {
        return html`
            <header>
                <nav>
                    <span class="back">
                        <slot name="back"></slot>
                    </span>
                    <span class="close">
                        <slot name="close"></slot>
                    </span>
                </nav>
                <slot name="heading"></slot>
                <slot name="author-line"></slot>
            </header>
            <slot name="body"></slot>
        `;
    }
}
