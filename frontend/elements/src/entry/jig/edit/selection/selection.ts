import { LitElement, html, css, customElement } from "lit-element";
import "@elements/core/images/ui";
import "@elements/entry/jig/_common/bg";

const STR_TITLE_1 = "Add an Activity";
const STR_TITLE_2 = "Create your own digital learning path";
const STR_TITLE_3 = "Drag activities to your JIG";

@customElement("jig-edit-selection")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: block;
                    height: 100vh;
                    overflow-y: auto;
                    padding: 48px 56px;
                    box-sizing: border-box;
                }
                .header {
                    display: flex;
                    justify-content: space-between;
                }
                .title1 {
                    font-size: 32px;
                    font-weight: 900;
                    font-stretch: normal;
                    font-style: normal;
                    line-height: 1.25;
                    letter-spacing: normal;
                    text-align: left;
                    color: var(--dark-blue-4);
                    margin-bottom: 8px;
                }
                .title2 {
                    margin-bottom: 4px;
                    font-size: 24px;
                    font-weight: normal;
                    font-stretch: normal;
                    font-style: normal;
                    line-height: 1;
                    letter-spacing: normal;
                    text-align: left;
                    color: var(--dark-gray-6);
                }
                .title3 {
                    margin-top: 8px;
                    font-size: 16px;
                    font-weight: normal;
                    font-stretch: normal;
                    font-style: normal;
                    line-height: 1.25;
                    letter-spacing: normal;
                    text-align: left;
                    color: #4a4a4a;
                }

                .modules {
                    margin-top: 56px;
                    display: flex;
                    gap: 64px;
                    flex-wrap: wrap;
                }
            `,
        ];
    }

    render() {
        return html`
            <div class="header">
                <div class="title">
                    <div class="title1">${STR_TITLE_1}</div>
                    <div class="title2">${STR_TITLE_2}</div>
                    <div class="title3">${STR_TITLE_3}</div>
                </div>
                <slot name="help"></slot>
            </div>
            <div class="modules">
                <slot name="modules"> </slot>
            </div>
        `;
    }
}
