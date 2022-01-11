import { LitElement, html, css, customElement, property } from "lit-element";

@customElement("page-header")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: grid;
                    grid-template-columns: repeat(5, auto);
                    justify-content: space-between;
                    align-items: center;
                    padding: 0 20px;
                    height: 68px;
                }
                @media (min-width: 1920px) {
                    :host {
                        padding: 0 40px;
                        height: 88px;
                    }
                }
                .logo {
                    width: 85px;
                }
                @media (min-width: 1920px) {
                    .logo {
                        width: 115px;
                    }
                }
                nav {
                    display: flex;
                    height: 100%;
                    column-gap: 4px;
                }
                @media (min-width: 1920px) {
                    nav {
                        column-gap: 8px;
                    }
                }
                .donate {
                    display: grid;
                    place-content: center;
                }
                .user {
                    display: flex;
                    column-gap: 16px;
                    height: 100%;
                    align-items: center;
                }
            `,
        ];
    }

    render() {
        return html`
            <a href="/">
                <img-ui class="logo" path="core/page-header/logo.svg"></img-ui>
            </a>
            <nav>
                <slot name="links"></slot>
            </nav>
            <div class="donate">
                <slot name="donate"></slot>
            </div>
            <div class="student-code">
                <slot name="student-code"></slot>
            </div>
            <div class="user">
                <slot name="user"></slot>
            </div>
        `;
    }
}
