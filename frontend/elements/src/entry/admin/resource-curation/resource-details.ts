import { LitElement, html, css, customElement } from "lit-element";

@customElement("admin-resource-details")
export class _ extends LitElement {
    static styles = [
        css`
            :host {
                display: grid;
                grid-template-columns: 500px auto;
                padding: 10px;
                justify-content: center;
                grid-gap: 20px;
                margin-top: 40px;
            }
            .heading {
                display: flex;
                justify-content: space-between;
                align-items: center;
                grid-column: 1 / span 2;
            }
            .general-summary {
                height: 100%;
                color: var(--dark-gray-5);
            }
            ::slotted([slot="buttons"]) {
                display: flex;
                gap: 0 32px;
            }
            .input-container,
            .right-side {
                max-height: calc(100dvh - 220px);
                overflow: auto;
                box-sizing: border-box;
            }
            .input-container {
                padding: 31px 24px;
                border-radius: 12px;
                border: solid 2px #e6f0ff;
            }
            ::slotted([slot="inputs"]) {
                display: grid;
                grid-template-rows: repeat(5, auto) 200px 200px;
                gap: 24px 0;
            }
            .right-side {
                display: grid;
                row-gap: 20px;
                align-self: start;
                box-shadow: #00000054 0 0 2px 0;
                padding: 10px;
            }
        `
    ];

    render() {
        return html`
            <div class="heading">
                <div>
                    <div class="general-summary">General Summary</div>
                    <slot name="back"></slot>
                </div>
                <div class="heading-buttons">
                    <slot name="buttons"></slot>
                </div>
            </div>
            <div class="input-container">
                <slot name="inputs"></slot>
            </div>
            <div class="right-side">
                <slot name="open"></slot>
                <slot name="rating"></slot>
                <slot name="block"></slot>
            </div>
            <slot name="loader"></slot>
        `;
    }
}