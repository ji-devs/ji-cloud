import { LitElement, html, css, customElement } from "lit-element";

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

@customElement("player-popup")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    position: fixed;
                    top: 0;
                    left: 0;
                    z-index: 9999;
                    display: grid;
                    height: 100vh;
                    width: 100vw;
                }
                ::slotted(*) {
                    grid-row: 1;
                    grid-column: 1;
                }
                ::slotted([slot="iframe"]) {
                    height: 100%;
                    width: 100%;
                    background-color: #ffffff;
                    border: 0;
                }
                ::slotted([slot="close"]) {
                    justify-self: end;
                    align-self: start;
                    margin: 16px;
                    background-color: var(--dark-blue-8);
                    border: solid 1px var(--light-blue-3);
                    border-radius: 50%;
                    color: #ffffff;
                    height: 40px;
                    width: 40px;
                    cursor: pointer;
                    font-size: 32px;
                    line-height: 1em;
                    font-family: "OpenSans-light";
                    z-index: 1;
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
            <slot name="iframe"></slot>
            <slot name="close"></slot>
        `;
    }
}
