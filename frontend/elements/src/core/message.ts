import { LitElement, html, css, customElement, property } from "lit-element";

const STR_HEADER_OOPS = "Oops!";
const STR_HEADER = "There is a problem";
const STR_RELOAD = "Please reload the page";

@customElement("page-message")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    position: fixed;
                    height: 100vh;
                    width: 100vw;
                    display: grid;
                    place-content: center;
                    background-color: #ffffff;
                }
                .main {
                    display: grid;
                    justify-content: center;
                    row-gap: 64px;
                    border-radius: 16px;
                    padding: 64px;
                    justify-items: center;
                }
                h1 {
                    font-weight: 500;
                    font-size: 32px;
                    color: var(--dark-blue-5);
                }
            `,
        ];
    }

    @property({ type: String })
    text: string = "";

    @property({ type: String })
    image?: string;

    render() {
        return html`
            <div class="main">
                <h1>${this.text}</h1>
                ${this.image && html`<img-ui path="${this.image}"></img-ui>`}
                <slot name="action"></slot>
            </div>
        `;
    }
}

