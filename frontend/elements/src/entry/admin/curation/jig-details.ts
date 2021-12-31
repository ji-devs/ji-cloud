import { LitElement, html, css, customElement } from "lit-element";

@customElement("admin-curation-jig-details")
export class _ extends LitElement {
    static styles = [
        css`
            #container {
                max-width: 800px;
                padding: 44px;
                background-color: white;
            }
            #heading {
                display: flex;
                justify-content: space-between;
                align-items: center;
            }
            #general-summary {
                height: 100%;
                color: var(--dark-gray-5);
            }
            ::slotted([slot="buttons"]) {
                display: flex;
                gap: 0 32px;
            }
            #input-container {
                margin-top: 12px;
                padding: 31px 24px;
                border-radius: 12px;
                border: solid 2px #e6f0ff;
            }
            ::slotted([slot="inputs"]) {
                display: flex;
                flex-direction: column;
                gap: 24px 0;
            }
        `
    ];

    render() {
        return html`
            <div id="container">
                <div id="heading">
                    <div>
                        <div id="general-summary">General Summary</div>
                        <slot name="back"></slot>
                    </div>
                    <div id="heading-buttons">
                        <slot name="buttons"></slot>
                    </div>
                </div>
                <div id="input-container">
                    <slot name="inputs"></slot>
                </div>
            </div>
            <slot name="loader"></slot>
        `;
    }
}