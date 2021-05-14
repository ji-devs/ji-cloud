import { LitElement, html, css, customElement, property } from 'lit-element';
import "@elements/core/images/ui";


@customElement('creator-publish-add-resource')
export class _ extends LitElement {
    static get styles() {
        return [css`
            :host {
                position: relative;
            }
            .add-button {
                display: inline-flex;
                cursor: pointer;
                align-items: center;
                column-gap: 8px;
            }
            .circle {
                width: 32px;
                height: 32px;
                display: inline-grid;
                place-content: center;
                border-radius: 50%;
                background-color: var(--light-blue-3);
                color: var(--main-blue);
                font-size: 24px;
                font-weight: 300;
            }
            :host(:hover) .circle {
                background-color: var(--main-blue);
                color: white;
            }
            .label {
                font-size: 13px;
                font-weight: 500;
                color: var(--dark-gray-6);
            }
            :host(:hover) .label {
                color: var(--main-blue);
            }
            .popup {
                display: none;
                position: absolute;
                z-index: 1;
                padding: 30px;
                border-radius: 8px;
                box-shadow: 0 3px 16px 0 rgba(0, 0, 0, 0.2);
                background-color: var(--white);
                left: 40px;
                min-width: 245px;
            }
            :host(:hover) .popup {
                display: block;
            }
      `];
    }

    @property()
    label: string = "";

    render() {
        return html`
            <div class="add-button">
                <span class="circle">
                    +
                </span>
                <span class="label">${this.label}</span>
            </div>
            <div class="popup">
                <slot name="add-method"></slot>
            </div>
        `;
    }
}
