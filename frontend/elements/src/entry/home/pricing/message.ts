import { LitElement, html, css, customElement, property } from "lit-element";

@customElement("pricing-message")
export class _ extends LitElement {
    static get styles() {
        return [css`
            :host {
                height: 196px;
                width: 292px;
                position: relative;
                display: block;
            }
            .talk {
                margin: auto;
                color: var(--dark-blue-4);
                display: block;
                border-radius: 16px;
                height: 120px;
                width: 200px;
                display: grid;
                place-content: center;
                text-align: center;
                position: relative;
                padding: 0px 20px;
                box-sizing: border-box;
            }
            .talk::after {
                content: "";
                top: 100%;
                left: calc(50% - 30px);
                position: absolute;
                bottom: 0px;
                border-right: 30px solid transparent;
                box-sizing: border-box;
            }
            :host([discount]) .talk::after {
                border-top-color: var(--green-3);
            }
            .talk h5 {
                margin: 0;
                font-size: 18px;
                font-weight: 700;
                color: var(--dark-blue-4);
            }
            .talk p {
                margin: 0;
                font-size: 14px;
                font-weight: 400;
                line-height: 1.4;
                color: var(--dark-blue-4);
            }
            img-ui {
                height: 88px;
                overflow: hidden;
                object-fit: cover;
                object-position: 9px;
                width: 160px;
                position: absolute;
                z-index: 2;
                margin-top: -12px;
                margin-left: -25px;
            }
        `];
    }

    @property()
    color: string = "";

    @property()
    title: string = "";

    @property()
    message: string = "";

    render() {
        return html`
            <style>
                .talk {
                    background-color: ${this.color};
                }
                .talk::after {
                    border-top: 12px solid ${this.color};
                }
            </style>
            <span class="talk">
                <h5>${this.title}</h5>
                <p>${this.message}</p>
            </span>
            <img-ui class="jigling-image" path="entry/home/pricing/jigling.png"></img-ui>
        `;
    }
}
