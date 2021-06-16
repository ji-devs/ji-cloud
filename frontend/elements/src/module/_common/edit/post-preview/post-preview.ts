import { LitElement, html, css, customElement, property } from 'lit-element';
import '@elements/core/images/ui';
import { GET_STR_MODULE, ModuleKind } from '@elements/entry/jig/module-types';

const STR_ACTION_HEADER = "What do you want to do next?";
const STR_USE_IN_ACTION_HEADER = "Use the content from this memory game in:";

const STR_MESSAGE_LINE_1_PART_1 = "Your ";
const STR_MESSAGE_LINE_1_PART_2 = " is ready!";
const STR_MESSAGE_LINE_2 = "It’s now part of your JIG.";

@customElement('post-preview')
export class _ extends LitElement {
    static get styles() {
        return [css`
            :host {
                height: 800px;
                max-width: 1232px;
                display: grid;
                grid-template-rows: 440px 360px;
                box-shadow: 0 3px 6px 0 rgba(0, 0, 0, 0.16);
                border-radius: 32px;
                overflow: hidden;
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
            ::slotted([slot=action-1of3]) {
                grid-column: 1;
            }
            ::slotted([slot=action-matching]) {
                grid-column: 2;
            }
            ::slotted([slot=action-flashcards]) {
                grid-column: 3;
            }
            .divider {
                width: 1px;
                background-color: var(--light-orange-6);
                grid-column: 4;
                height: 112px;
            }
            ::slotted([slot=action-print]) {
                grid-column: 5;
            }
            ::slotted([slot=action-continue]) {
                grid-column: 6;
            }
        `];
    }

    @property()
    module: ModuleKind = "memory";

    render() {
        return html`
            <div class="top-section">
                <img-ui path="module/_common/post-preview/splash.png"></img-ui>
                <div class="message">
                    ${STR_MESSAGE_LINE_1_PART_1}
                    ${GET_STR_MODULE(this.module)}
                    ${STR_MESSAGE_LINE_1_PART_2}
                    <br>
                    ${STR_MESSAGE_LINE_2}
                </div>
            </div>
            <div class="bottom-section">
                <h3 class="action-header">${STR_ACTION_HEADER}</h3>
                <h4 class="action-use-in-header">${STR_USE_IN_ACTION_HEADER}</h4>
                <slot class="slot-1of3" name="action-1of3"></slot>
                <slot class="slot-matching" name="action-matching"></slot>
                <slot class="slot-flashcards" name="action-flashcards"></slot>
                <div class="divider"></div>
                <slot class="slot-print" name="action-print"></slot>
                <slot class="slot-continue" name="action-continue"></slot>
            </div>
        `;
    }
}
