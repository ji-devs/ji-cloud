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
                    grid-template-columns: 480px 1fr;
                    height: 100vh;
                }
                aside {
                    height: 100%;
                    background-color: #def4ff;
                }
                img-ui {
                    width: 100%;
                    height: 100%;
                    object-fit: cover;
                    display: block;
                }
                .main {
                    box-sizing: border-box;
                    padding: 80px;
                    display: flex;
                    flex-direction: column;
                    gap: 20px;
                    height: 100%;
                    overflow: auto;
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
