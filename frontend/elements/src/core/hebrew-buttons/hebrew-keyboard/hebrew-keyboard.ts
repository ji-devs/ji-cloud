import {
    LitElement,
    html,
    css,
    customElement,
    property,
    TemplateResult,
    query,
} from "lit-element";
import { render as renderNiqqud, styles as buttonStyles } from "./niqqud";
import { render as renderLetter, styles as letterStyles } from "./letter";
import {
    render as renderPunctuation,
    styles as punctuationStyles,
} from "./punctuation";
import { letters, niqquds, punctuations } from "./data";
import { enderStyles, renderDelete, renderEnder, renderSpace } from "./buttons";
import { cantillationsStyles, renderCantillations } from "./cantillations";
import { AnchoredOverlayAbsolute } from "@elements/core/overlays/anchored-overlay-absolute";
import "@elements/core/icon/fa-icon";

const STR_DRAG_ME = "Drag me around";

@customElement("hebrew-keyboard")
export class _ extends LitElement {
    static get styles() {
        return [
            buttonStyles,
            letterStyles,
            punctuationStyles,
            cantillationsStyles,
            enderStyles,
            css`
                :host {
                    display: grid;
                    justify-items: center;
                    filter: drop-shadow(0 3px 10px rgba(0, 0, 0, 0.16));
                    cursor: grab;
                }
                .main {
                    padding: 12px;
                    display: inline-grid;
                    grid-template-rows: repeat(5, 32px);
                    row-gap: 8px;
                    border-radius: 14px;
                    background-color: #fff;
                }
                .row {
                    display: grid;
                    grid-auto-rows: 32px;
                    row-gap: 8px;
                    column-gap: 6px;
                    direction: rtl;
                }
                .row:nth-child(1) {
                    grid-template-columns: 72px repeat(10, 32px) 46px;
                }
                .row:nth-child(2) {
                    grid-template-columns: 48px repeat(12, 32px);
                }
                .row:nth-child(3) {
                    grid-template-columns: 42px repeat(11, 36px);
                }
                .row:nth-child(4) {
                    grid-template-columns: 48px repeat(12, 32px);
                }
                .row:nth-child(5) {
                    grid-template-columns: 112px 260px repeat(3, 36px);
                }
                .tet,
                .fay {
                    /* make room for enter button */
                    grid-column: 2;
                }
                button {
                    color: var(--dark-gray-6);
                    text-align: center;
                    font-size: 20px;
                    background-color: var(--light-blue-1);
                    border-radius: 4px;
                    border: solid 1px #e7f0fd;
                    height: 32px;
                    line-height: 32px;
                    display: grid;
                    place-content: center;
                    padding: 0;
                    box-sizing: border-box;
                    cursor: pointer;
                    position: relative;
                }
                button.image-button {
                    display: grid;
                }
                button.image-button .letter-placeholder {
                    grid-row: 1;
                    grid-column: 1;
                    display: inline-block;
                    height: 14px;
                    width: 14px;
                    border-radius: 2px;
                    border: dotted 1px var(--light-blue-5);
                    grid-row: 1;
                    grid-column: 1;
                    place-self: center;
                }
                button.image-button img-ui {
                    grid-row: 1;
                    grid-column: 1;
                }
                anchored-overlay-absolute button[slot="anchor"] {
                    width: 100%;
                }
                anchored-overlay-absolute[open] button[slot="anchor"] {
                    background-color: var(--light-blue-3);
                }
                .tooltip {
                    display: none;
                    padding: 8px 12px;
                    box-shadow: 0 3px 6px 0 rgba(0, 0, 0, 0.08);
                    border: solid 1px #fdd994;
                    background-color: var(--light-orange-3);
                    position: absolute;
                    color: var(--dark-gray-6);
                    font-size: 14px;
                    line-height: 1em;
                    border-radius: 16px;
                    bottom: calc(100% + 10px);
                    white-space: nowrap;
                    z-index: 5;

                    /* https://stackoverflow.com/a/25776315/5253155 */
                    left: 50%;
                    transform: translateX(-50%);
                }
                button:hover span.tooltip {
                    display: block;
                }
                .tooltip::after {
                    content: "";
                    display: inline-block;
                    height: 10px;
                    width: 10px;
                    position: absolute;
                    border-bottom: solid 1px #fdd994;
                    border-right: solid 1px #fdd994;
                    background-color: var(--light-orange-3);

                    left: 50%;
                    top: calc(100% - 5px);
                    transform: translateX(-50%) rotate(45deg);
                }
                anchored-overlay-absolute [slot="overlay"] {
                    background-color: #ffffff;
                    padding: 8px;
                    box-sizing: border-box;
                    border-radius: 4px;
                    box-shadow: 0 3px 6px 0 rgba(0, 0, 0, 0.16);
                    display: flex;
                    column-gap: 8px;
                }
                anchored-overlay-absolute [slot="overlay"] button {
                    width: 32px;
                }
                .under {
                    background-color: var(--main-blue);
                    clip-path: path("M 200 0 c -2 18 -1 24 -13 24 L 14 24 c -12 0 -12 -5 -14 -24 z");
                    width: 200px;
                    font-size: 14px;
                    line-height: 24px;
                    text-align: center;
                    color: #ffffff;
                }
            `,
        ];
    }

    private onClick = (char: string) => {
        const input = this.deepActiveElementOrWysiwyg() as any;
        if ("setRangeText" in input) {
            input.setRangeText(
                char,
                input.selectionStart,
                input.selectionEnd,
                "end"
            );
        } else if ("setTextAtSelection" in input) {
            // for wysiwyg
            input.setTextAtSelection(char);
        }
        input.dispatchEvent(new Event("input"));
        input.dispatchEvent(new Event("change"));
    };

    // TODO: enable this one and remove the other one once we bring back shadow root to wysiwyg base
    // private deepActiveElementOrWysiwyg() {
    //     let a = document.activeElement;
    //     while (
    //         a &&
    //         !a.matches("wysiwyg-base") &&
    //         a.shadowRoot &&
    //         a.shadowRoot.activeElement
    //     ) {
    //         a = a.shadowRoot.activeElement;
    //     }
    //     return a;
    // }
    private deepActiveElementOrWysiwyg() {
        let a = document.activeElement;
        let wysiwyg = a?.closest("wysiwyg-base");
        if(wysiwyg)
            return wysiwyg;
        return a;
    }

    private onDelete = () => {
        const input = this.deepActiveElementOrWysiwyg() as any;
        if ("setRangeText" in input) {
            if (input.selectionStart === input.selectionEnd) {
                const start = input.selectionStart;
                if (start === 0) return;
                input.value =
                    input.value.slice(0, start - 1) + input.value.slice(start);
                input.setSelectionRange(start - 1, start - 1);
            } else {
                input.setRangeText(
                    "",
                    input.selectionStart,
                    input.selectionEnd,
                    "end"
                );
            }
        } else if ("triggerBackspace" in input) {
            // for wysiwyg
            input.triggerBackspace();
        }
        input.dispatchEvent(new Event("input"));
        input.dispatchEvent(new Event("change"));
    };

    @query("anchored-overlay-absolute#cantillations")
    private cantillationsOverlay!: AnchoredOverlayAbsolute;

    private toggleCantillationsOpen = () => {
        this.cantillationsOverlay.open = !this.cantillationsOverlay.open;
    };

    renderRow1(): TemplateResult[] {
        let elements = [];
        elements.push(renderDelete(this.onDelete));
        elements = elements.concat(
            letters.slice(0, 11).map((letter) => {
                return renderLetter(letter, this.onClick);
            })
        );
        return elements;
    }

    renderRow2(): TemplateResult[] {
        let elements = [];
        elements.push(renderEnder(this.onClick));
        elements = elements.concat(
            letters.slice(11, 23).map((letter) => {
                return renderLetter(letter, this.onClick);
            })
        );
        return elements;
    }

    renderRow3(): TemplateResult[] {
        let elements: any[] = [];
        elements.push(html`<span></span>`);
        elements = elements.concat(
            letters.slice(23).map((letter) => {
                return renderLetter(letter, this.onClick);
            })
        );
        return elements;
    }

    renderRow4(): TemplateResult[] {
        return niqquds.map((niqqud) => {
            return renderNiqqud(niqqud, this.onClick);
        });
    }

    renderRow5(): TemplateResult[] {
        let elements: any[] = [];
        elements.push(
            renderCantillations(this.onClick, this.toggleCantillationsOpen)
        );
        elements.push(renderSpace(this.onClick));
        elements = elements.concat(
            punctuations.map((punctuation) => {
                return renderPunctuation(punctuation, this.onClick);
            })
        );
        return elements;
    }

    connectedCallback() {
        super.connectedCallback();
        this.tabIndex = 0;
        this.addEventListener("focusin", this.onFocus);
    }
    disconnectedCallback() {
        super.disconnectedCallback();
        this.removeEventListener("focusin", this.onFocus);
    }
    private onFocus(e: FocusEvent) {
        let relatedTarget = e.relatedTarget as HTMLElement | null;

        while (relatedTarget) {
            relatedTarget.focus();
            if (relatedTarget.matches(":focus")) {
                break;
            } else {
                relatedTarget = relatedTarget.shadowRoot?.querySelector(
                    "input, textarea, [contentediable]"
                ) as HTMLElement | null;
            }
        }
    }

    render() {
        return html`
            <div class="main">
                <div class="row">${this.renderRow1()}</div>
                <div class="row">${this.renderRow2()}</div>
                <div class="row">${this.renderRow3()}</div>
                <div class="row">${this.renderRow4()}</div>
                <div class="row">${this.renderRow5()}</div>
            </div>
            <div class="under">
                <fa-icon icon="fa-solid fa-grip-dots"></fa-icon>
                ${STR_DRAG_ME}
            </div>
        `;
    }
}
