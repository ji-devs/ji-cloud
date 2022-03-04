import { LitElement, html, css, customElement, property } from "lit-element";

@customElement("page-header")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    position: relative;
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
                .beta {
                    position: absolute;
                    background-color: var(--dark-blue-5);
                    color: #ffffff;
                    left: 0;
                    bottom: 0;
                    transform: translateY(100%);
                    border-radius: 0 0 12px 12px;
                    margin-left: 1em;
                }

                /* mobile */
                @media (max-width: 1000px) {
                    :host {
                        justify-content: center;
                    }
                    nav, .donate, .student-code, .user {
                        display: none;
                    }
                    .beta {
                        left: 50%;
                        transform: translate(-50%, 100%);
                        margin-left: 0;
                    }
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
            <span class="beta">
                <slot name="beta"></slot>
            </span>
        `;
    }
}
