import { LitElement, html, css, customElement, property } from "lit-element";

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
                    background-color: #ffffff;
                    border-radius: 16px;
                    box-shadow: rgb(0 0 0 / 25%) 0px 3px 16px 0px;
                    padding: 30px;
                    justify-items: center;
                }
                h1 {
                    font-weight: 600;
                    font-size: 30px;
                }
            `,
        ];
    }

    render() {
        return html`
            <div class="main">
                <h1>Oops... We crashed, that's embarrassing 😳</h1>
                <button-rect @click="${() => location.reload()}">
                    Reload
                    <!-- <fa-icon icon="fa-regular fa-rotate-right"></fa-icon> -->
                </button-rect>
            </div>
        `;
    }
}
