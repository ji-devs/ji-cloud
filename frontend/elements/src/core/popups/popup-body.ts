import { LitElement, html, css, customElement, property } from 'lit-element';
import "@elements/core/images/ui";
import "@elements/core/progress-bar/progress-bar";

@customElement("popup-body")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: grid;
                    border-radius: 16px;
                    box-shadow: 0 3px 16px 0 rgba(0, 0, 0, 0.25);
                    background-color: #ffffff;
                }
                header {
                    padding: 16px;
                }
                nav {
                    display: flex;
                    justify-content: space-between
                }
                .inner {
                    padding: 16px;
                }
                ::slotted([slot=back]) {
                    /* only add margin in back exists */
                    margin-bottom: 16px;
                }
                ::slotted([slot=close]) {
                    font-size: 24px;
                    color: var(--dark-gray-5);
                    font-weight: 300;
                }
                ::slotted([slot=heading]) {
                    color: #fd7076;
                    font-size: 24px;
                    font-weight: 600;
                    margin: 0 16px;
                    padding-bottom: 16px;
                    border-bottom: solid 1px #d5e4ff;
                }
            `
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
            </header>
            <slot name="body"></slot>
        `;
    }
}
