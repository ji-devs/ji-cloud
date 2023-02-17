import { LitElement, html, css, customElement, property } from "lit-element";

const STR_CREATE_ACCOUNT = "Create a FREE Jigzi account!";
const STR_SIGNUP = "Sign up";
const STR_ALREADY_HAVE_ACCOUNT = "Already have an account?";
const STR_LOGIN = "Log in";

const STR_WHAT_ARE_YOU_WAITING = "What are you waiting for?";
const STR_HUGE_LIBRARY = "A huge library of interactive educational activities";
const STR_CREATE_SIMPLY = "Create your own JIGs quickly and simply";
const STR_SHARE_TO_GOOGLE_CLASSROOM = "Easily share through Google classroom";

@customElement("home-login-before-play")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                .overlay {
                    position: fixed;
                    top: 0;
                    left: 0;
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    width: 100vw;
                    height: 100dvh;
                    opacity: 0.8;
                    background-color: var(--light-blue-3);
                    z-index: 10;
                }
                .body {
                    position: fixed;
                    top: 50%;
                    left: 50%;
                    transform: translate(-50%, -50%);
                    z-index: 10;

                    width: 880px;
                    display: grid;
                    grid-template-columns: 50% 50%;
                    background: #fff;
                    border-radius: 16px;
                    box-shadow: 0 3px 16px 0 rgba(0, 0, 0, 0.16);
                    overflow: hidden;
                }
                .left-side {
                    grid-column: 1;
                    grid-row: 1;
                    padding: 32px;
                    display: grid;
                    grid-template-rows: auto auto 1fr;
                }
                .left-side h1 {
                    color: var(--dark-blue-4);
                    font-size: 24px;
                    font-weight: bold;
                    /* margin: 0; */
                }
                .left-side .divider {
                    height: 1px;
                    background-color: #d5e4ff;
                }
                .left-side .actions {
                    display: grid;
                    justify-content: center;
                    align-content: center;
                }
                .right-side {
                    grid-column: 2;
                    grid-row: 1;
                    background-color: var(--light-blue-6);
                    color: #fff;
                    padding: 24px;
                }
                .right-side img-ui {
                    height: 300px;
                    display: grid;
                    place-content: center;
                }
                .right-side ul {
                    list-style: none;
                    padding: 0;
                }
                .right-side ul li {
                    display: grid;
                    grid-template-columns: 24px 1fr;
                    padding: 0;
                    font-size: 14px;
                }
                ::slotted([slot=close]) {
                    grid-column: 2;
                    grid-row: 1;
                    justify-self: end;
                    align-self: start;
                    padding: 10px;
                }
            `,
        ];
    }

    render() {
        return html`
            <div class="overlay"></div>
            <div class="body">
                <div class="left-side">
                    <h1>${STR_CREATE_ACCOUNT}</h1>
                    <div class="divider"></div>
                    <div class="actions">
                        <button-rect href="/user/signup">${STR_SIGNUP}</button-rect>
                        <p>
                            ${STR_ALREADY_HAVE_ACCOUNT}
                            <button-rect kind="text" href="/user/login">${STR_LOGIN}</button-rect>
                        </p>
                    </div>
                </div>
                <div class="right-side">
                    <img-ui path="entry/home/search-results/jigglings.webp"></img-ui>
                    <h4>${STR_WHAT_ARE_YOU_WAITING}</h4>
                    <ul>
                        <li><fa-icon icon="fa-solid fa-check"></fa-icon>${STR_HUGE_LIBRARY}</li>
                        <li><fa-icon icon="fa-solid fa-check"></fa-icon>${STR_CREATE_SIMPLY}</li>
                        <li><fa-icon icon="fa-solid fa-check"></fa-icon>${STR_SHARE_TO_GOOGLE_CLASSROOM}</li>
                    </ul>
                </div>
                <slot name="close"></slot>
            </div>
        `;
    }
}
