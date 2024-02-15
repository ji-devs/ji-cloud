import { LitElement, html, css, customElement } from "lit-element";

@customElement("kids-student-code-name")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: grid;
                    justify-content: center;
                    background-color: var(--green-3);
                    height: 100dvh;
                    width: 100vw;
                    box-sizing: border-box;
                }
                main {
                    display: grid;
                    row-gap: 45px;
                    grid-column: 1;
                    grid-row: 1;
                    width: 100vw;
                    align-content: start;
                    z-index: 1;
                }
                header {
                    background-color: white;
                    height: 88px;
                    display: grid;
                    align-items: center;
                    justify-content: start;
                    padding: 0 25px;
                }
                .center {
                    width: 580px;
                    max-width: 80vw;
                    display: grid;
                    margin: auto;
                }
                h1 {
                    margin: 0;
                    margin-top: 80px;
                    font-size: 32px;
                    font-weight: 900;
                    color: var(--dark-blue-4);
                    text-align: center;
                }
                ::slotted(input) {
                    border-radius: 14px;
                    border: solid 1px var(--main-blue);
                    background-color: var(--white);
                    font-size: 28px;
                    font-weight: 500;
                    padding: 20px 24px;
                    margin-top: 48px;
                }
                ::slotted(input)::placeholder {
                    color: var(--light-gray-4);
                }
                ::slotted([slot=clear]) {
                    justify-self: end;
                    background-color: transparent;
                    width: 100px;
                    height: 48px;
                    border: solid 3px var(--main-blue);
                    border-left: 0;
                    border-radius: 14px 12px 12px 14px;
                    color: var(--main-blue);
                    font-size: 20px;
                    font-weight: 600;
                    cursor: pointer;
                    display: flex;
                    align-items: center;
                    justify-content: space-evenly;
                    margin-top: 32px;
                }
                ::slotted([slot=clear]):hover,
                ::slotted([slot=clear]):active {
                    color: var(--dark-blue-2);
                    border-color: var(--dark-blue-2);
                }
                ::slotted([slot=clear])::before {
                    content: "";
                    display: inline-block;
                    width: 31px;
                    height: 31px;
                    transform: rotate(45deg) translate(-29px, 29px);
                    border-left: solid 3px var(--main-blue);
                    border-bottom: solid 3px var(--main-blue);
                    border-radius: 5px 0 5px 8px;
                    position: absolute;
                }
                ::slotted([slot=clear]):hover::before,
                ::slotted([slot=clear]):active::before {
                    border-color: var(--dark-blue-2);
                }
                ::slotted([slot=clear]) .icon {
                    font-size: 30px;
                }
                ::slotted([slot=play]) {
                    margin-top: 94px;
                    width: 224px;
                    justify-self: center;
                }
            `,
        ];
    }

    render() {
        return html`
            <main>
                <header>
                    <img-ui path="core/page-header/logo.svg"></img-ui>
                </header>
                <div class="center">
                    <h1>Your name השם שלך</h1>
                    <slot name="input"></slot>
                    <slot name="clear"></slot>
                    <slot name="play"></slot>
                </div>
            </main>
        `;
    }
}
