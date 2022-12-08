import { LitElement, html, css, customElement } from "lit-element";

const STR_ADD_WORDS = "Add words here";

@customElement("sidebar-widget-dual-list")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                .lists {
                    width: 100%;
                    display: grid;
                    grid-template-columns: 1fr 1fr;
                    overflow: auto;
                    max-height: calc(100% - 150px);
                    scrollbar-color: #d3d4dd transparent;
                    gap: 6px;
                }
                .header {
                    top: 0;
                    position: sticky;
                    background-color: #e9eff8;
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
                    align-items: center;
                }

                .input-buttons {
                    padding: 12px 0;
                    display: flex;
                    justify-content: flex-end;
                }

                .done-btn {
                    display: flex;
                    justify-content: flex-end;
                    padding: 16px 0;
                    bottom: 0;
                    position: sticky;
                    background-color: #e9eff8;
                }
            `,
        ];
    }

    render() {
        return html`
            <div class="header">
                <header>
                    <div>${STR_ADD_WORDS}</div>
                    <div><slot name="clear"></slot></div>
                </header>
                <div class="input-buttons">
                    <slot name="hebrew-buttons"></slot>
                </div>
            </div>
            <div class="lists-and-actions">
                <div class="lists">
                    <slot name="left"></slot>
                    <slot name="right"></slot>
                </div>
                <div class="done-btn">
                    <slot name="done-btn"></slot>
                </div>
                <slot name="error"></slot>
            </div>
        `;
    }
}
