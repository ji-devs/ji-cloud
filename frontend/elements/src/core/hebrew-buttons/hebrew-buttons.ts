import { LitElement, html, css, customElement, property, internalProperty } from "lit-element";
import { nothing } from "lit-html";
import { classMap } from "lit-html/directives/class-map";
import "../drag/container";
import "./hebrew-keyboard/hebrew-keyboard";

type Button = "sefaria" | "dicta" | "keyboard";

const KEYBOARD_HEIGHT = 216;

@customElement("hebrew-buttons")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                .main {
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
                :host(:not([full]):not(:hover)) .full-only {
                    display: none;
                }
            `,
        ];
    }

    @property({ type: Boolean })
    full: boolean = false;

    @internalProperty()
    active?: Button;

    private rect?: DOMRect;

    private onButtonClick(button: Button) {
        if(this.active === button) {
            this.active = undefined;
        } else {
            this.active = button;
            this.rect = this.getBoundingClientRect();
        }
    }

    private renderHebrewKeyboard() {
        return html`
            <drag-container x="0" y="${this.rect!.top - KEYBOARD_HEIGHT}">
                <hebrew-keyboard></hebrew-keyboard>
            </drag-container>
        `;
    }

    private renderButton(button: Button, fullOnly: boolean) {
        const classes = classMap({
            "active": this.active === button,
            "full-only": fullOnly,
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

    render() {
        return html`
            <div class="main">
                ${this.renderButton("sefaria", true)}
                <div class="divider full-only"></div>
                ${this.renderButton("dicta", true)}
                <div class="divider full-only"></div>
                ${this.renderButton("keyboard", false)}
            </div>
            ${
                this.active === "keyboard" ? this.renderHebrewKeyboard()
                    : nothing
            }
        `;
    }
}
