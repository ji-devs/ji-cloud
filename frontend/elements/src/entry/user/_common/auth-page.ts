import {
    LitElement,
    html,
    css,
    customElement,
    property,
} from "lit-element";

@customElement("auth-page")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: grid;
                    height: 100vh;
                    justify-content: center;
                }
                @media (min-width: 1920px) {
                    :host {
                        grid-template-columns: 480px 1fr;
                    }
                }
                aside {
                    height: 100%;
                    background-color: #def4ff;
                    display: none;
                }
                @media (min-width: 1920px) {
                    aside {
                        display: block;
                    }
                }
                img-ui {
                    width: 100%;
                    height: 100%;
                    object-fit: cover;
                    display: block;
                }
                .main {
                    box-sizing: border-box;
                    display: flex;
                    flex-direction: column;
                    gap: 20px;
                    height: 100%;
                    overflow: auto;
                    padding: 30px;
                }
                @media (min-width: 1920px) {
                    .main {
                        padding: 80px;
                    }
                }
            `,
        ];
    }

    @property()
    img: string = "";

    render() {
        return html`
            <aside>
                <img-ui .path="${this.img}"></img-ui>
            </aside>
            <div class="main">
                <slot></slot>
            </div>
        `;
    }
}
