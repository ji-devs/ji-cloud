import { LitElement, html, css, customElement } from "lit-element";

const STR_ADD_WORDS = "Add Your Words";
const STR_INPUT_FOOTER = "2 to 14 words";

@customElement("sidebar-widget-dual-list")
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
                .lists {
                    width: 100%;
                    display: grid;
                    grid-template-columns: 1fr 1fr;
                    overflow: auto;
                    max-height: calc(100% - 150px);
                    scrollbar-color: #d3d4dd transparent;
                    gap: 6px;
                }
                @media (min-width: 1920px) {
                    .lists {
                        gap: 12px;
                    }
                }

                /* weird mechanism to get scrollbar margin-left https://stackoverflow.com/a/45420691/5253155 */
                .lists::-webkit-scrollbar {
                    width: 9px; /* 6px, plus 3px for border */
                }
                .lists::-webkit-scrollbar-track {
                    box-shadow: inset 0 0 9px 9px transparent;
                    border-left: solid 3px transparent;
                }
                .lists::-webkit-scrollbar-thumb {
                    box-shadow: inset 0 0 9px 9px #d3d4dd;
                    border-left: solid 3px transparent;
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
                <div class="lists">
                    <slot name="left"></slot>
                    <slot name="right"></slot>
                </div>
                <div class="input-footer">${STR_INPUT_FOOTER}</div>
                <div class="done-btn">
                    <slot name="done-btn"></slot>
                </div>
                <slot name="error"></slot>
            </div>
        `;
    }
}
