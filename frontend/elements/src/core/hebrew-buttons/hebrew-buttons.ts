import { LitElement, html, css, customElement, property, internalProperty, query } from "lit-element";
import { nothing } from "lit-html";
import { classMap } from "lit-html/directives/class-map";
import "../drag/container";
import "./hebrew-keyboard/hebrew-keyboard";
import "@elements/core/overlays/anchored-overlay";

type Button = "sefaria" | "dicta" | "keyboard";

export const KEYBOARD_HEIGHT = 216;

const SEFARIA_URL = "https://www.sefaria.org/texts/Tanakh";
const DICTA_URL = "https://embed.dicta.org.il/";

@customElement("hebrew-buttons")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                anchored-overlay {
                    display: block;
                }
                #buttons-wrapper {
                    display: inline-flex;
                    grid-gap: 12px;
                    align-items: center;
                }
                button {
                    background-color: transparent;
                    border: 0;
                    cursor: pointer;
                    padding: 0;
                    display: grid;
                    place-content: center;
                }
                button img-ui {
                    display: none;
                    height: 28px;
                }
                button:not(.active):not(:hover) .img-default {
                    display: block;
                }
                button:not(.active):hover .img-hover {
                    display: block;
                }
                button.active .img-active {
                    display: block;
                }
                .divider {
                    width: 1px;
                    height: 20px;
                    background-color: var(--main-blue);
                }
                .no-short {
                    display: none;
                }
                :host([full]) .no-short, :host(:hover) .no-short, .full .no-short {
                    display: inline-block;
                }
                .iframe-wrapper {
                    height: 665px;
                }
                :host([active=sefaria]) .iframe-wrapper {
                    width: 700px;
                }
                :host([active=dicta]) .iframe-wrapper {
                    width: 550px;
                }
                .close {
                    position: absolute;
                    background-color: var(--main-blue);
                    color: #ffffff;
                    height: 32px;
                    width: 32px;
                    font-size: 26px;
                    border-radius: 50%;
                    transform: translate(50%, -50%);
                    z-index: 1;
                }
                :host([active=sefaria]) .iframe-close {
                    /* offset by width of iframe minus 50% of the button width */
                    transform: translate(calc(700px - 50%), -50%);
                }
                :host([active=dicta]) .iframe-close {
                    /* offset by width of iframe minus 50% of the button width */
                    transform: translate(calc(550px - 50%), -50%);
                }
                .keyboard-close {
                    right: 0;
                }
                .iframe-wrapper iframe {
                    height: 100%;
                    width: 100%;
                    border: 0;
                }
            `,
        ];
    }

    @property({ type: Boolean })
    full: boolean = false;

    @property({ reflect: true })
    active?: Button;

    @query("#buttons-wrapper")
    private buttonsWrapper!: HTMLElement;

    private rect?: DOMRect;

    // this can be overridden to change the keyboard placement position
    public positionKeyboard(rect: DOMRect): { x: number, y: number } {
        return {
            x: rect.right,
            y: rect.top,
        }
    }

    private onButtonClick(button: Button) {
        if(this.active === button) {
            this.active = undefined;
        } else {
            this.active = button;
            this.rect = this.buttonsWrapper.getBoundingClientRect();
        }
    }

    private renderHebrewKeyboard() {
        const pos = this.positionKeyboard(this.rect!);

        return html`
            <drag-container y="${pos.y}" x="${pos.x}">
                <button class="close keyboard-close" @click=${() => this.active = undefined}>&times;</button>
                <hebrew-keyboard></hebrew-keyboard>
            </drag-container>
        `;
    }

    private renderButton(button: Button, noShort: boolean) {
        const classes = classMap({
            "active": this.active === button,
            "no-short": noShort,
        });

        return html`
            <button
                type="button"
                @click="${() => this.onButtonClick(button)}"
                class="${classes}"
            >
                <img-ui class="img-default" path="core/hebrew-buttons/${button}.svg"></img-ui>
                <img-ui class="img-hover" path="core/hebrew-buttons/${button}-hover.svg"></img-ui>
                <img-ui class="img-active" path="core/hebrew-buttons/${button}-active.svg"></img-ui>
            </button>
        `;
    }

    private getIframeUrl(): string {
        return this.active === "sefaria" ? SEFARIA_URL
            : this.active === "dicta" ? DICTA_URL
            : "";
    }

    render() {
        const classes = classMap({
            "full": Boolean(this.active),
        });

        return html`
            <anchored-overlay
                positionY="bottom-out"
                positionX="left-in"
                .styled=${true}
                .autoClose=${false}
                ?open=${this.active === "dicta" || this.active === "sefaria"}
            >
                <div slot="anchor" id="buttons-wrapper" part="buttons-wrapper" class="${classes}">
                    ${this.renderButton("sefaria", true)}
                    <div class="divider no-short"></div>
                    ${this.renderButton("dicta", true)}
                    <div class="divider no-short"></div>
                    ${this.renderButton("keyboard", false)}
                </div>
                <div slot="overlay" class="iframe-wrapper">
                    <iframe src="${this.getIframeUrl()}"></iframe>
                </div>
            </anchored-overlay>
            ${
                this.active === "dicta" || this.active === "sefaria" ? (
                        html`
                            <button
                                class="close iframe-close"
                                @click=${() => this.active = undefined}
                            >&times;</button>
                        `
                    ) : (
                        nothing
                    )
            }
            ${
                this.active === "keyboard" ? this.renderHebrewKeyboard()
                    : nothing
            }
        `;
    }
}
