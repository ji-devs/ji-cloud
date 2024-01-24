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
                    height: 100dvh;
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
                        grid-column: 1;
                        grid-row: 1;
                    }
                }
                aside img-ui {
                    width: 100%;
                    height: 100%;
                    max-height: 100dvh;
                    object-fit: cover;
                    display: block;
                }
                .logo {
                    grid-column: 1;
                    grid-row: 1;
                    margin: 30px;
                    align-self: start;
                    justify-self: center;
                }
                @media (min-width: 1024px) {
                    .logo {
                        justify-self: start;
                        display: block;
                    }
                }
                .main-wrapper {
                    height: 100%;
                    overflow: auto;
                    display: grid;
                    justify-content: center;
                }
                main {
                    box-sizing: border-box;
                    display: flex;
                    flex-direction: column;
                    gap: 16px;
                    padding: 20px;
                }
                @media (min-width: 1024px) {
                    main {
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
            <a class="logo" href="/">
                <img-ui path="core/page-header/logo.svg"></img-ui>
            </a>
            <div class="main-wrapper">
                <main>
                    <slot></slot>
                </main>
            </div>
        `;
    }
}
