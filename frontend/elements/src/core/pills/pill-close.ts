import { LitElement, html, css, customElement, property } from 'lit-element';

@customElement('pill-close')
export class _ extends LitElement {
    static get styles() {
        return [css`
            :host {
                border-radius: 12px;
                border: solid 1px #b0ccf2;
                background-color: #f3f8fe;
                display: inline-flex;
                justify-content: center;
                align-items: center;
                font-size: 14px;
                line-height: 24px;
                color: #387af4;
                position: relative;
                padding: 0 12px;
                box-sizing: border-box;
                position: relative;
            }
            :host(:hover) {
                border-color: #5893f9;
            }
            ::slotted([slot=delete]) {
                display: none;
            }
            :host(:hover) ::slotted([slot=delete]) {
                display: inline-block;
                position: absolute;
                right: -8px;
                top: -8px;
            }
        `];
    }

    @property()
    label: string = "";

    render() {
        return html`
            <span>${this.label}</span>
            <slot name="delete"></slot>
        `;
    }
}
