import { LitElement, html, css, customElement } from "lit-element";

@customElement("admin-curation-jig-details")
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
            .input-container {
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
            .player {
                display: grid;
                aspect-ratio: 16 / 9;
                width: 200px;
                align-items: center;
                justify-items: center;
            }
            ::slotted([slot=player]) {
                grid-column: 1;
                grid-row: 1;
            }
            /* ::slotted(img-module-screenshot[slot=player]) {
                width: 200px;
            } */
            ::slotted(fa-button[slot=player]) {
                font-size: 50px;
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
            <div class="player">
                <slot name="player"></slot>
            </div>
            <slot name="loader"></slot>
        `;
    }
}