import { LitElement, html, css, customElement, property, state } from "lit-element";

const CSS_RULE_WEBKIT = `
    :root::-webkit-scrollbar {
        display: none;
    }
`;
const CSS_RULE_FIREFOX = `
    :root {
        scrollbar-width: none;
    }
`;

const stylesheet = document.createElement("style");
document.head.appendChild(stylesheet);


@customElement("pro-dev-player")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: flex;
                    height: 100vh;
                    justify-content: center;
                    align-items: center;
                }
                main {
                    display: grid;
                    width: 70vw;
                    justify-content: center;
                    align-items: center;
                    background-color: var(--light-blue-3);
                    position: relative;
                }
              
                body {
                  display: flex;
                  flex-direction: row;
                  justify-content: center;
                }
              
                ::slotted([slot=player-window]) {
                    order: 0;
                    display: flex;
                    position: relative;
                    width: 40vw;
                    height: 40vw;
                    border: solid;
                    align-items: center; 
                    justify-content: center; 
                }

                ::slotted([slot=title]) {
                    order: 1;
                    display: grid;
                    position: relative;
                    justify-content: center;
                }

                ::slotted([slot=navigation]) {
                    order: 2;
                    display: grid;
                    position: relative;
                    justify-content: center;
                }

                ::slotted([slot=close]) {
                    position: absolute;
                    display: flex;
                    right: 0px;
                    top: 0;
                    padding: 12px;
                }
            `,
        ];
    }

    connectedCallback() {
        super.connectedCallback();

        stylesheet.sheet!.insertRule(CSS_RULE_WEBKIT);
        stylesheet.sheet!.insertRule(CSS_RULE_FIREFOX);
    }

    disconnectedCallback() {
        super.disconnectedCallback();

        stylesheet.sheet!.deleteRule(0);
        stylesheet.sheet!.deleteRule(0);
    }

    render() {
        return html`
            <main>
                <body>
                    <slot name="player-window"></slot>
                    <slot name="title"></slot>
                    <slot name="navigation"></slot>
                </body>
                <div>
                    <slot name="close"></slot>
                </div>
            </main>
        `;
    }
}
