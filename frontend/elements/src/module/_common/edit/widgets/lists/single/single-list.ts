import { LitElement, html, css, customElement } from "lit-element";

const STR_ADD_WORDS = "Add Your Words";
const STR_INPUT_FOOTER = "2 to 14 words";

@customElement("sidebar-widget-single-list")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: grid;
                    /*
                        using minmax(0, 1fr) instead of just 1fr to allow the items inside to overflow without growing the container.
                        https://stackoverflow.com/a/52861514/5253155
                        https://stackoverflow.com/a/43312314/5253155
                    */
                    grid-template-rows: auto auto minmax(0, 1fr);
                    height: 100%;
                }
                header {
                    display: flex;
                    justify-content: space-between;
                }
                .input-buttons {
                    margin-top: 34px;
                    display: flex;
                    justify-content: flex-end;
                    margin-bottom: 12px;
                    margin-right: 4px;
                }
                @media (min-width: 1920px) {
                    .input-buttons {
                        margin-bottom: 18px;
                        margin-right: 0px;
                    }
                }
                .input-footer {
                    font-size: 16px;
                    text-align: center;
                    color: var(--light-blue-5);
                    margin-top: 12px;
                }
                @media (min-width: 1920px) {
                    .input-footer {
                        margin-top: 24px;
                    }
                }
                .list {
                    box-sizing: border-box;
                    border-radius: 16px;
                    border: solid 2px var(--light-blue-4);
                    background-color: var(--white);
                    display: flex;
                    flex-direction: column;
                    overflow: auto;
                    max-height: calc(100% - 150px);
                    scrollbar-color: #d3d4dd transparent;
                    scrollbar-width: thin;
                }
                .list::-webkit-scrollbar {
                    width: 6px;
                }
                .list::-webkit-scrollbar-track {
                    background-color: transparent;
                }
                .list::-webkit-scrollbar-thumb {
                    border-radius: 3px;
                    background-color: #d3d4dd;
                }
                .list::-webkit-scrollbar-button {
                    background-color: transparent;
                    height: 8px;
                }

                .list ::slotted(*:not(:last-child)) {
                    border-bottom: solid 1px var(--light-blue-4);
                }
                .done-btn {
                    display: flex;
                    justify-content: flex-end;
                    margin-top: 16px;
                    padding-bottom: 40px;
                }
            `,
        ];
    }

    render() {
        return html`
            <header>
                <div>${STR_ADD_WORDS}</div>
                <div><slot name="clear"></slot></div>
            </header>
            <div class="input-buttons">
                <slot name="hebrew-buttons"></slot>
            </div>
            <div class="lists-and-actions">
                <div class="list">
                    <slot></slot>
                </div>
                <div class="input-footer">${STR_INPUT_FOOTER}</div>
                <div class="done-btn">
                    <slot name="done-btn"></slot>
                </div>
            </div>
        `;
    }
}
