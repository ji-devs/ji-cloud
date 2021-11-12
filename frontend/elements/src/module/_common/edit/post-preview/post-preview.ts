import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/images/ui";
import {
    ModuleKind,
    STR_MODULE_DISPLAY_NAME,
} from "@elements/module/_common/types";

const STR_ACTION_HEADER = "What do you want to do next?";
const STR_USE_IN_PREFIX = "Use the content from this";
const STR_USE_IN_SUFFIX = "activity in:";

const STR_HEADER_LINE_1_PREFIX = "Your";
const STR_HEADER_LINE_1_SUFFIX = "activity is ready!";
const STR_HEADER_LINE_2 = "It’s now part of your JIG.";

@customElement("post-preview")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    height: 800px;
                    width: 90%;
                    max-width: 1232px;
                    display: grid;
                    grid-template-rows: 440px 360px;
                    box-shadow: 0 3px 6px 0 rgba(0, 0, 0, 0.16);
                    border-radius: 32px;
                    overflow: auto;
                    margin: 20px 0;
                    max-height: calc(100vh - 40px);
                }
                .top-section {
                    background-color: #fff;
                    display: grid;
                    grid-template-rows: auto min-content;
                    justify-content: center;
                    padding: 60px 0;
                    justify-items: center;
                }
                .message {
                    text-align: center;
                    line-height: 1.18;
                    font-size: 22px;
                    font-weight: 500;
                    color: #fd7076;
                    margin: 0;
                }
                .bottom-section {
                    background-color: var(--light-orange-1);
                    display: grid;
                    grid-template-columns: repeat(3, 116px) 1px repeat(2, 116px);
                    column-gap: 48px;
                    justify-content: center;
                    align-items: center;
                    padding: 46px 0;
                }
                .bottom-section-centered {
                    background-color: var(--light-orange-1);
                    display: flex;
                    flex-direction: column;
                    column-gap: 48px;
                    justify-content: center;
                    align-items: center;
                    padding: 46px 0;
                }
                .action-header {
                    color: #fd7076;
                    font-size: 32px;
                    grid-column: 1 / -1;
                    text-align: center;
                    margin: 0;
                    margin-bottom: 24px;
                    font-weight: 900;
                }
                .action-use-in-header {
                    grid-column: 1 / span 3;
                    text-align: center;
                    color: #4a4a4a;
                    margin: 0;
                    margin-bottom: 12px;
                    font-weight: 500;
                }
                ::slotted([slot="module-1"]) {
                    grid-column: 1;
                }
                ::slotted([slot="module-2"]) {
                    grid-column: 2;
                }
                ::slotted([slot="module-3"]) {
                    grid-column: 3;
                }
                .divider {
                    width: 1px;
                    background-color: var(--light-orange-6);
                    grid-column: 4;
                    height: 112px;
                }
                ::slotted([slot="action-print"]) {
                    grid-column: 5;
                }
                ::slotted([slot="action-continue"]) {
                    grid-column: 6;
                }
            `,
        ];
    }

    @property()
    module: ModuleKind = "memory";

    @property({ type: Boolean })
    hasConvertable: boolean = false;

    render() {
        const { module, hasConvertable } = this;

        return html`
            <div class="top-section">
                <img-ui
                    path="module/_common/edit/post-preview/splash.png"
                ></img-ui>
                <div class="message">
                    ${STR_HEADER_LINE_1_PREFIX}
                    ${STR_MODULE_DISPLAY_NAME[this.module]}
                    ${STR_HEADER_LINE_1_SUFFIX}
                    <br />
                    ${STR_HEADER_LINE_2}
                </div>
            </div>
            ${hasConvertable
                ? renderConvertable(module)
                : renderNonConvertable()}
            <slot name="loader"></slot>
        `;
    }
}

function renderNonConvertable() {
    return html`
        <div class="bottom-section-centered">
            <h3 class="action-header">${STR_ACTION_HEADER}</h3>
            <slot class="action-print" name="action-print"></slot>
            <slot class="action-continue" name="action-continue"></slot>
        </div>
    `;
}
function renderConvertable(module: ModuleKind) {
    return html`
        <div class="bottom-section">
            <h3 class="action-header">${STR_ACTION_HEADER}</h3>
            <h4 class="action-use-in-header">
                ${STR_USE_IN_PREFIX} ${STR_MODULE_DISPLAY_NAME[module]}
                ${STR_USE_IN_SUFFIX}
            </h4>
            <slot class="module-1" name="module-1"></slot>
            <slot class="module-2" name="module-2"></slot>
            <slot class="module-3" name="module-3"></slot>
            <div class="divider"></div>
            <slot class="action-print" name="action-print"></slot>
            <slot class="action-continue" name="action-continue"></slot>
        </div>
    `;
}
