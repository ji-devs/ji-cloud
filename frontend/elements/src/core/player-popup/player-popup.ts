import { LitElement, html, css, customElement, property } from "lit-element";
import { nothing } from "lit-html";

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

export type Size = "aspect-ratio" | "full-screen";

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
                    height: 100dvh;
                    width: 100vw;
                    background-color: #ececec;
                }
                .iframe-wrapper {
                    background-color: #ffffff;
                    border: 0;
                    box-shadow: 0 3px 16px 0 rgba(0, 0, 0, 0.16);
                    position: relative;
                    top: 50%;
                    transform: translateY(-50%);
                    margin: 0 auto;
                }
                :host([size=aspect-ratio]) .iframe-wrapper {
                    max-height: 100dvh;
                    max-width: 100vw;
                    aspect-ratio: 16 / 9;
                }
                :host([size=full-screen]) .iframe-wrapper {
                    height: 100dvh;
                    width: 100vw;
                }
                .iframe-wrapper ::slotted(iframe) {
                    height: 100%;
                    width: 100%;
                    aspect-ratio: 16 / 9;
                    border: 4px red solid;
                }
                ::slotted([slot="close"]) {
                    margin: 16px;
                    border-radius: 50%;
                    color: #ffffff;
                    height: 40px;
                    width: 40px;
                    cursor: pointer;
                    font-size: 32px;
                    line-height: 1em;
                    font-family: "OpenSans-light";
                    z-index: 1;
                    position: absolute;
                    top: 0;
                    right: 0;
                    padding: 0;
                }

                :host .preview {
                    position: absolute;
                    left: 50%;
                    display: inline;
                    transform: translateX(-50%);
                    padding: 2px 29px 4px;
                    box-shadow: 0 3px 6px 0 rgba(0, 0, 0, 0.16);
                    border: solid 1px var(--dark-blue-3);
                    background-color: var(--dark-blue-3);
                    border-radius: 0 0 16px 16px;
                    color: var(--white);
                }
            `,
        ];
    }

    @property({ reflect: true })
    size: Size = "aspect-ratio";

    @property({ type: Boolean })
    preview: boolean = false;

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
                ${this.preview ? html`<div class="preview">Preview mode</div>` : nothing}
                <slot name="iframe"></slot>
                <slot name="close"></slot>
            </div>
        `;
    }
}
