import { LitElement, html, css, customElement, property, TemplateResult, query } from 'lit-element';
import { render as renderNiqqud, styles as buttonStyles } from './niqqud';
import { render as renderLetter, styles as letterStyles } from './letter';
import { render as renderPunctuation, styles as punctuationStyles } from './punctuation';
import { letters, niqquds, punctuations } from './data';
import { enderStyles, renderDelete, renderEnder, renderSpace } from './buttons';
import { cantillationsStyles, renderCantillations } from './cantillations';
import { AnchoredOverlay } from '@elements/core/overlays/anchored-overlay';

@customElement('hebrew-keyboard')
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
                    padding: 12px;
                    display: grid;
                    grid-template-rows: repeat(5, 32px);
                    row-gap: 8px;
                    border-radius: 14px;
                    box-shadow: 0 3px 6px 0 rgba(0, 0, 0, 0.16);
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
                .tet, .fay {
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
                anchored-overlay button[slot=anchor] {
                    width: 100%;
                }
                anchored-overlay[open] button[slot=anchor] {
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
                    content: '';
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
                anchored-overlay [slot=overlay] {
                    background-color: #ffffff;
                    padding: 8px;
                    box-sizing: border-box;
                    border-radius: 4px;
                    box-shadow: 0 3px 6px 0 rgba(0, 0, 0, 0.16);
                    display: flex;
                    column-gap: 8px;
                }
                anchored-overlay [slot=overlay] button {
                    width: 32px;
                }
            `
        ];
    }

    private onClick = (char: string) => {
        const input = this.deepActiveElement() as any;
        if("setRangeText" in input) {
            input.setRangeText(char, input.selectionStart, input.selectionEnd, "end");
        }
    }

    private deepActiveElement() {
        let a = document.activeElement;
        while (a && a.shadowRoot && a.shadowRoot.activeElement) {
            a = a.shadowRoot.activeElement;
        }
        return a;
    }

    private onDelete = () => {
        const input = this.deepActiveElement() as any;
        if("setRangeText" in input) {
            if(input.selectionStart === input.selectionEnd) {
                const start = input.selectionStart;
                if (start === 0) return;
                input.value = input.value.slice(0, start - 1) + input.value.slice(start);
                input.setSelectionRange(start - 1, start - 1);
            } else {
                input.setRangeText('', input.selectionStart, input.selectionEnd, "end");
            }
        }
    }

    @query("anchored-overlay#cantillations")
    private cantillationsOverlay!: AnchoredOverlay;

    private toggleCantillationsOpen = () => {
        this.cantillationsOverlay.open = !this.cantillationsOverlay.open;
    }

    renderRow1(): TemplateResult[] {
        let elements = [];
        elements.push(renderDelete(this.onDelete));
        elements = elements.concat(letters.slice(0, 11).map(letter => {
            return renderLetter(letter, this.onClick);
        }));
        return elements;
    }

    renderRow2(): TemplateResult[] {
        let elements = [];
        elements.push(renderEnder(this.onClick));
        elements = elements.concat(letters.slice(11, 23).map(letter => {
            return renderLetter(letter, this.onClick);
        }));
        return elements;
    }

    renderRow3(): TemplateResult[] {
        let elements: any[] = [];
        elements.push(html`<span></span>`);
        elements = elements.concat(letters.slice(23).map(letter => {
            return renderLetter(letter, this.onClick);
        }));
        return elements;
    }

    renderRow4(): TemplateResult[] {
        return niqquds.map(niqqud => {
            return renderNiqqud(niqqud, this.onClick);
        });
    }

    renderRow5(): TemplateResult[] {
        let elements: any[] = [];
        elements.push(renderCantillations(this.onClick, this.toggleCantillationsOpen));
        elements.push(renderSpace(this.onClick));
        elements = elements.concat(punctuations.map(punctuation => {
            return renderPunctuation(punctuation, this.onClick);
        }));
        return elements;
    }


    connectedCallback() {
        super.connectedCallback();
        this.tabIndex = 0;
        this.addEventListener("focusin", this.onFocus)
    }
    disconnectedCallback() {
        super.disconnectedCallback();
        this.removeEventListener("focusin", this.onFocus)
    }
    private onFocus(e: FocusEvent) {
        (e.relatedTarget as any)?.focus();
    }

    render() {
        return html`
            <div class="row">
                ${this.renderRow1()}
            </div>
            <div class="row">
                ${this.renderRow2()}
            </div>
            <div class="row">
                ${this.renderRow3()}
            </div>
            <div class="row">
                ${this.renderRow4()}
            </div>
            <div class="row">
                ${this.renderRow5()}
            </div>
        `;
    }
}
