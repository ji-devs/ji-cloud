import { LitElement, html, css, customElement, property } from 'lit-element';

@customElement('pill-close')
export class _ extends LitElement {
    static get styles() {
        return [css`
            .wrapper {
                border-radius: 12px;
                border: solid 1px #b0ccf2;
                background-color: #f3f8fe;
                display: inline-flex;
                justify-content: center;
                align-items: center;
                font-size: 14px;
                height: 24px;
                color: #387af4;
                position: relative;
                padding: 5px 12px;
                box-sizing: border-box;
            }

            img-ui {
                position: absolute;
                top: -7px;
                left: 80px;
                display: none;
                height: 16px;
                width: 16px;
            }

            .wrapper:hover img-ui {
                display: block;
                cursor: pointer;
            }

            .negative {
                border: solid 1px #6ea3f9;
                color: #afcbf4;
                background-color: inherit;
            }
        `];
    }
    @property({ type: Boolean })
    negative: boolean = false;

    @property()
    label: string = "";

    render() {
        const { negative, label } = this;

        return html`
            <div class="wrapper ${negative ? 'negative' : ''}">
                <span>${label}</span>
                <img-ui path="icn-delete-tab.svg"></img-ui>
            </div>
        `;
    }
}
