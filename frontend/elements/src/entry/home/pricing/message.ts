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
                background-color: var(--light-orange-3);
            }
            :host([discount]) .talk {
                background-color: var(--green-3);
            }
            .talk::after {
                content: "";
                top: 100%;
                left: calc(50% - 30px);
                position: absolute;
                bottom: 0px;
                border-right: 30px solid transparent;
                box-sizing: border-box;
                border-top: 12px solid var(--light-orange-3);
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

    @property({ type: Boolean, reflect: true })
    discount: boolean = false;

    @property()
    title: string = "20% OFF";

    @property()
    message: string = "special launch prices valid until 13.05.23";

    render() {
        return html`
            <span class="talk">
                <h5>choose Jigzi!</h5>
                <p>Join the thousands of educators around the world who</p>
            </span>
            <img-ui class="jigling-image" path="temp/pricing-jigling.png"></img-ui>
        `;
    }
}
