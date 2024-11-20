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
                    background-color: #ffffff;
                }
                @media (min-width: 1920px) {
                    :host {
                        padding: 0 40px;
                        height: 88px;
                    }
                }
                .logo-anchor {
                    margin-top: 12px;
                    margin-right: 12rem;
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
                .end {
                    display: grid;
                    grid-template-columns: repeat(2, auto);
                    height: 100%;
                    column-gap: 16px;
                    justify-content: space-between;
                }
                .end .help {
                    display: grid;
                    place-content: center;
                }
                .end .help ::slotted([slot="help"]){
                    display: grid;
                    place-content: center;
                    padding: .5em;
                    color: none;
                }
                .end .help ::slotted([slot="help"]:hover) {
                    place-content: center;
                    border-radius: 50%;
                    background-color: #c4d9f7;
                }
                .end .user {
                    display: flex;
                    column-gap: 16px;
                    height: 100%;
                    justify-content: end;
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
                a {
                    color: inherit;
                }
                /* mobile */
                @media (max-width: 1023px) {
                    :host {
                        justify-content: center;
                    }
                    nav, .student-code, .help, .user {
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

    @property()
    href: string = "";

    @property()
    target: string = "";

    @property()
    icon: string = "";


    render() {
        return html`
            <a class="logo-anchor" href="/">
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
            <div class="end">
                <div class="help">
                    <a href= "/home/help" color="black">
                        <slot name="help"></slot>
                    </a>
                </div>
                <div class="user">
                    <slot name="user"></slot>
                </div>
            </div>
            <span class="beta">
                <slot name="beta"></slot>
            </span>
        `;
    }
}
