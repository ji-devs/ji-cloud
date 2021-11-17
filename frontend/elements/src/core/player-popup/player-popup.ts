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
                    place-content: center;
                    height: 100vh;
                    width: 100vw;
                    background-color: #ececec;
                }
                ::slotted(*) {
                    grid-row: 1;
                    grid-column: 1;
                }
                .iframe-wrapper {
                    grid-row: 1;
                    grid-column: 1;
                    height: 100%;
                    width: 100%;
                    max-height: 100vh;
                    max-width: 100vw;
                    background-color: #ffffff;
                    border: 0;
                    aspect-ratio: 16 / 9;
                    box-shadow: 0 3px 16px 0 rgba(0, 0, 0, 0.16);
                }
                .iframe-wrapper ::slotted(iframe) {
                    height: 100%;
                    width: 100%;
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
            <div class="iframe-wrapper">
                <slot name="iframe"></slot>
            </div>
            <slot name="close"></slot>
        `;
    }
}
