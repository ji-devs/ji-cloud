import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/core/images/ui";
import {
    ModuleKind,
    STR_MODULE_DISPLAY_NAME,
} from "@elements/module/_common/types";

const STR_ACTION_HEADER = "What do you want to do next?";
const STR_USE_IN= "Use this content in:";

const STR_HEADER_LINE_1_PREFIX = "Your";
const STR_HEADER_LINE_1_SUFFIX = "activity is ready!";

@customElement("post-preview")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    width: 90%;
                    max-width: 740px;
                    display: grid;
                    grid-template-rows: 1fr;
                    box-shadow: 0 3px 6px 0 rgba(0, 0, 0, 0.16);
                    border-radius: 32px;
                    overflow: auto;
                    margin: 20px 0;
                }
                @media screen and (max-width: 1280px) {
                    .bottom-section .action-header, .bottom-section-centered .action-header {
                        font-size: 24px;
                    }
                    .bottom-section .message, .bottom-section-centered .message {
                        font-size: 20px;
                    }
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
                    background-color: var(--white);
                    display: grid;
                    align-items: center;
                    align-content: center;
                    padding: 40px 0;
                }
                .bottom-section .actions {
                    display: flex;
                    flex-direction: column;
                    justify-content: center;
                    grid-gap: 15px;
                }
                .bottom-section .actions .middle-plane {
                    background-color: var(--light-orange-1);
                    height: 220px;
                    padding-bottom: 12px;
                }
                .bottom-section .actions > div {
                    display: grid;
                    grid-template-columns: repeat(auto-fit, 116px);
                    justify-content: center;
                    column-gap: 48px;
                }
                .bottom-section-centered .actions {
                    display: grid;
                    grid-auto-flow: column;
                    justify-content: center;
                    column-gap: 48px;
                }
                .bottom-section-centered {
                    background-color: var(--white);
                    display: grid;
                    justify-content: center;
                    align-items: center;
                    padding: 50px 0;
                }
                .action-header {
                    color: #fd7076;
                    font-size: 22px;
                    grid-column: 1 / -1;
                    text-align: center;
                    margin: 0;
                    margin-bottom: 30px;
                    font-weight: 900;
                }
                .action-use-in-header {
                    grid-column: 1 / span 3;
                    text-align: center;
                    color: #4a4a4a;
                    margin: auto;
                    margin-bottom: 10px;
                    font-weight: 500;
                }
                .bottom-section .actions ::slotted([slot="module-1"]) {
                    grid-column: 1;
                }
                .bottom-section .actions ::slotted([slot="module-2"]) {
                    grid-column: 2;
                }
                .bottom-section .actions ::slotted([slot="module-3"]) {
                    grid-column: 3;
                }
                .bottom-section .actions .divider {
                    width: 1px;
                    background-color: var(--light-orange-6);
                    grid-column: 4;
                    height: 112px;
                .bottom-section .actions ::slotted([slot="action-print"]) {
                    grid-column: 5;
                }
                .bottom-section .actions ::slotted([slot="action-publish"]) {
                    grid-column: 6;
                }
                .bottom-section .actions ::slotted([slot="action-continue"]) {
                    grid-column: 7;
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
            ${hasConvertable
                ? renderConvertable(module)
                : renderNonConvertable(module)}
            <slot name="loader"></slot>
        `;
    }
}

function renderMessage(module: ModuleKind) {
    return html`
        <div class="message">
            ${STR_HEADER_LINE_1_PREFIX}
            ${STR_MODULE_DISPLAY_NAME[module]}
            ${STR_HEADER_LINE_1_SUFFIX}
        </div>
    `;
}

function renderNonConvertable(module: ModuleKind) {
    return html`
        <div class="bottom-section-centered">
            ${renderMessage(module)}
            <h3 class="action-header">${STR_ACTION_HEADER}</h3>
            <div class="actions">
                <slot class="action-continue" name="action-continue"></slot>
                <slot class="action-print" name="action-print"></slot>
                <slot class="action-publish" name="action-publish"></slot>
            </div>
        </div>
    `;
}
function renderConvertable(module: ModuleKind) {
    return html`
        <div class="bottom-section">
            ${renderMessage(module)}
            <h3 class="action-header">${STR_ACTION_HEADER}</h3>
            <div class="actions">
                <div class= "middle-plane">
                    <h4 class="action-use-in-header">
                        ${STR_USE_IN}
                    </h4>
                    <slot class="module-1" name="module-1"></slot>
                    <slot class="module-2" name="module-2"></slot>
                    <slot class="module-3" name="module-3"></slot>
                </div>
                <div>
                    <slot class="action-continue" name="action-continue"></slot>
                    <slot class="action-print" name="action-print"></slot>
                    <slot class="action-publish" name="action-publish"></slot>
                </div>
            </div>
        </div>
    `;
}
