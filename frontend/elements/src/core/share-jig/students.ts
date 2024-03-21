import {
    LitElement,
    html,
    css,
    customElement,
    property,
    PropertyValues,
    state,
} from "lit-element";
import "@elements/core/popups/popup-body";
import "@elements/core/buttons/rectangle";
import { nothing } from "lit-html";
import { classMap } from "lit-html/directives/class-map";

const STR_CODE_HEADER = "Share with Code";

const formatter = new Intl.DateTimeFormat(undefined, {
    year: "numeric",
    month: "long",
  	day: "numeric",
});


@customElement("share-jig-students")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    background-color: #ffffff;
                }
                popup-body {
                    border-radius: 16px;
                    box-shadow: rgb(0 0 0 / 25%) 0px 3px 16px 0px;
                    background-color: #ffffff;
                }
                .body {
                    padding: 0 32px 32px 32px;
                    display: grid;
                    row-gap: 8px;
                    width: 305px;
                    min-height: 250px;
                    align-content: start;
                }
                ::slotted([slot=name-input]) {
                    border: solid 1px var(--main-blue);
                    background-color: var(--white);
                    border-radius: 8px;
                    font-size: 20px;
                    padding: 10px;
                }
                label {
                    display: grid;
                    font-weight: 14px;
                }
                label strong {
                    font-weight: 600;
                }
                label a {
                    color: var(--main-blue);
                    font-weight: 600;
                    text-decoration: none;
                }
                input {
                    background-color: var(--light-blue-2);
                    border-radius: 8px;
                    padding: 14px 18px;
                    font-size: 16px;
                    font-weight: 500;
                    color: var(--dark-gray-6);
                    border: 0;
                    width: 100%;
                    box-sizing: border-box;
                }
                .field-code input {
                    font-size: 32px;
                    font-weight: 300;
                    text-align: center;
                    padding-top: 32px;
                }
                .field-code .under {
                    display: flex;
                    justify-content: space-between;
                }
                .field-code .valid-until {
                    color: #4a4a4a;
                    font-size: 14px;
                }
                .field-code ::slotted([slot="copy-code"]) {
                    grid-column: 2;
                }
                .field-code .code-illustration {
                    display: grid;
                    position: relative;
                    justify-content: center;
                }
                .field-code .code-illustration span {
                    font-size: 13px;
                    font-weight: 600;
                    position: absolute;
                    top: 100%;
                    left: 50%;
                    translate: -50% 0;
                    background-color: var(--light-blue-4);
                    border-bottom-left-radius: 12px;
                    border-bottom-right-radius: 12px;
                    padding: 2px 6px;
                }
                .field-url .under {
                    display: flex;
                    justify-content: flex-end;
                    align-items: center;
                    column-gap: 8px;
                }
            `,
        ];
    }

    @property()
    url: string = "";

    @property()
    code?: string;

    @property({ type: Number })
    secondsToExpire?: number;

    @state()
    exprDateLabel?: string;

    updated(changedProperties: PropertyValues) {
        if (changedProperties.has("secondsToExpire")) {
            this.exprUpdated();
        }
    }

    private exprUpdated() {
        if (this.secondsToExpire) {
            const date = new Date();
            date.setSeconds(date.getSeconds() + this.secondsToExpire);
            this.exprDateLabel = formatter.format(date);
        } else {
            this.exprDateLabel = "";
        }
    }

    render() {
        return html`
            <popup-body
                class=${classMap({
                    "no-code": this.code === "",
                })}
            >
                <slot slot="back" name="back"></slot>
                <slot slot="close" name="close"></slot>
                <h3 slot="heading">${STR_CODE_HEADER}</h3>
                <div slot="body" class="body">
                    ${ !this.code ? html`
                        <slot name="name-input"></slot>
                        <slot name="gen-code-button"></slot>
                    ` : html`
                        <div class="field-code">
                            <label>
                                <span>Go to <a href="/">Jigzi.org</a> and input this code:</span>
                                <div class="code-illustration">
                                    <img-ui path="core/page-header/kids.svg"></img-ui>
                                    <span>Student Code</span>
                                </div>
                                <input readonly value="${this.code}" />
                            </label>
                            <div class="under">
                                <span class="valid-until">
                                    ${this.secondsToExpire ? html`
                                        Valid until ${this.exprDateLabel}
                                    ` : nothing}
                                </span>
                                <slot name="copy-code"></slot>
                            </div>
                        </div>
                        <div class="field-url">
                            <label>
                                <span><strong>OR</strong> go straight to link:</span>
                                <input readonly value="${this.url}" />
                            </label>
                            <div class="under">
                                <slot name="copy-url"></slot>
                            </div>
                        </div>
                        <slot name="qr"></slot>
                    ` }
                </div>
            </popup-body>
        `;
    }
}
