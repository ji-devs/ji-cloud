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
                @media (min-width: 1024px) {
                    :host {
                        grid-template-columns: 360px 1fr;
                    }
                }
                aside {
                    height: 100%;
                    background-color: #def4ff;
                    display: none;
                }
                @media (min-width: 1024px) {
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
                    gap: 16px;
                    height: 100%;
                    overflow: auto;
                    padding: 20px;
                }
                @media (min-width: 1024px) {
                    .main {
                        padding: 45px;
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
