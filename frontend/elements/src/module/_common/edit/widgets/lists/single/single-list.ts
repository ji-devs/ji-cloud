import { LitElement, html, css, customElement } from "lit-element";

const STR_ADD_WORDS = "Add words here";

@customElement("sidebar-widget-single-list")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                header {
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                }
                .input-buttons {
                    padding: 10px 0;
                    display: flex;
                    justify-content: flex-end;
                }
                .header {
                    top: 0;
                    position: sticky;
                    background-color: #e9eff8;
                    font-size: 14px;
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
                    padding: 12px 0;
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
                <div class="list">
                    <slot></slot>
                </div>
                <div class="done-btn">
                    <slot name="done-btn"></slot>
                </div>
            </div>
        `;
    }
}
