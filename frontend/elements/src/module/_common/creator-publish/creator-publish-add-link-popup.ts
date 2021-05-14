import { LitElement, html, css, customElement, property } from 'lit-element';
import "@elements/core/images/ui";
import "@elements/core/overlays/anchored-overlay";

const STR_HEADER = "Enter URL";
const STR_INPUT_LABEL = "Enter your link here";

@customElement('creator-publish-add-link-popup')
export class _ extends LitElement {
    static get styles() {
        return [css`
            anchored-overlay::part(overlay) {
                border-radius: 16px;
                box-shadow: 0 3px 16px 0 rgba(0,0,0,0.25);
            }
            .popup {
                display: grid;
                padding: 24px 32px;
                width: 420px;
            }
            .nav-line {
                display: grid;
                grid-template-columns: auto auto;
                justify-content: space-between;
                height: 0px;
            }
            ::slotted([slot=dismiss-action]) {
                grid-column: 2;
                display: inline-block;
                height: 14px;
                width: 14px;
                margin-right: -12px;
                margin-top: -6px;
            }
            h4 {
                color: #fd7076;
                font-size: 24px;
                font-weight: 600;
                margin: 0;
                display: flex;
                align-items: center;
                column-gap: 8px;
            }
            .divider {
                background-color: #d5e4ff;
                height: 1px;
                margin: 16px 0;
            }
            label {
                font-weight: 500;
                color: #4a4a4a;
                display: grid;
                grid-gap: 8px;
            }
            ::slotted([slot=textarea]) {
                background-color: #f7f7f7;
                border-radius: 8px;
                padding: 11px 16px;
                border: 0;
                color: var(--dark-gray-6);
                resize: none;
                font-family: Poppins;
                box-sizing: border-box;
                width: 100%;
                height: 100px;
            }
            .actions {
                margin-top: 24px;
                display: grid;
                grid-template-columns: auto auto;
                justify-content: space-between;
                align-items: center;
            }
      `];
    }

    @property({type: Boolean})
    open: boolean = false;

    render() {
        return html`
            <anchored-overlay ?open=${this.open} @close="${() => this.open = false}">
                <slot slot="anchor" name="anchor"></slot>
                <div slot="overlay" class="popup">
                    <span class="nav-line">
                        <slot name="dismiss-action"></slot>
                    </span>
                    <h4>
                        <img-ui path="module/_common/creator-publish/link-icon-pink.svg"></img-ui>
                        ${STR_HEADER}
                    </h4>
                    <div class="divider"></div>
                    <label>
                        <span>${STR_INPUT_LABEL}</span>
                        <slot name="textarea"></slot>
                    </label>
                    <div class="actions">
                        <slot name="action-cancel"></slot>
                        <slot name="action-save"></slot>
                    </div>
                </div>
            </anchored-overlay>
        `;
    }
}
