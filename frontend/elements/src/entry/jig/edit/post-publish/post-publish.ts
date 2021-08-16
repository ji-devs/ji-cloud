import { LitElement, html, css, customElement, property } from 'lit-element';
import '@elements/core/images/ui';
import { ModuleKind, STR_MODULE_DISPLAY_NAME} from '@elements/module/_common/types';

const STR_ACTION_HEADER = "What do you want to do next?";
const STR_HEADER = "Your JIG is live!";

@customElement('post-publish')
export class _ extends LitElement {
    static get styles() {
        return [css`
            :host {
                display: grid;
                place-content: center;
                padding: 0 80px;
                min-height: 100vh;
                grid-template-columns: 1fr;
            }
            main {
                height: 800px;
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
                column-gap: 48px;
                justify-content: center;
                align-items: center;
                padding: 46px 0;
            }
            .action-header {
                color: #fd7076;
                font-size: 32px;
                text-align: center;
                margin: 0;
                margin-bottom: 24px;
                font-weight: 900;
            }
            .actions {
                display: flex;
                grid-gap: 60px;
                justify-content: center;
            }
        `];
    }

    render() {
        return html`
            <main>
                <div class="top-section">
                    <img-ui path="module/_common/edit/post-preview/splash.png"></img-ui>
                    <div class="message">
                        ${STR_HEADER}
                    </div>
                </div>
                <div class="bottom-section">
                    <h3 class="action-header">${STR_ACTION_HEADER}</h3>

                    <div class="actions">
                        <slot name="actions"></slot>
                    </div>
                </div>
                <slot name="loader"></slot>
            </main>
        `;
    }
}
