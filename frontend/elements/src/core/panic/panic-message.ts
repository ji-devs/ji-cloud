import { LitElement, html, css, customElement, property } from "lit-element";

const STR_HEADER_OOPS = "Oops!";
const STR_HEADER = "There is a problem";
const STR_RELOAD = "Please reload the page";

@customElement("panic-message")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    position: fixed;
                    height: 100vh;
                    width: 100vw;
                    background-color: #00000040;
                    display: grid;
                    place-content: center;
                }
                .main {
                    background-color: red;
                    display: grid;
                    justify-content: center;
                    row-gap: 64px;
                    background-color: #ffffff;
                    border-radius: 16px;
                    box-shadow: rgb(0 0 0 / 25%) 0px 3px 16px 0px;
                    padding: 64px;
                    justify-items: center;
                }
                h1 {
                    font-weight: 500;
                    font-size: 32px;
                    color: var(--dark-blue-5);
                }
                h1 .oops {
                    font-weight: 700;
                }
            `,
        ];
    }

    render() {
        return html`
            <div class="main">
                <h1>
                    <span class="oops">${STR_HEADER_OOPS}</span>
                    ${STR_HEADER}
                </h1>
                <img-ui path="core/panic/disconnected.webp"></img-ui>
                <button-rect @click="${() => location.reload()}">
                    ${STR_RELOAD}
                </button-rect>
            </div>
        `;
    }
}
