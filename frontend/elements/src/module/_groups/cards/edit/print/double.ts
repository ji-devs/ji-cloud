import { loadFonts } from "@elements/_themes/themes";
import { LitElement, html, css, customElement, property } from "lit-element";
import { CardKind } from "./types";

@customElement("module-card-print-double")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: inline-grid;
                    grid-template-rows: 20px 30vw;
                    border-right: var(--border);
                    margin-top: 20px;
                }
                fa-icon {
                    color: #a1a8ad;
                    justify-self: end;
                    transform: translate(10px, -10px) rotate(90deg);
                }
                .card {
                    display: grid;
                    grid-template-columns: 45vw 45vw;
                    grid-auto-rows: minmax(0, 1fr);
                    border-top: var(--border);
                    border-bottom: var(--border);
                    border-left: var(--border);
                }
                .card-side:first-child {
                    border-right: solid 1px #a1a8ad;
                }
                .card-side {
                    display: grid;
                    grid-template-rows: 1fr min-content;

                    padding: 8px;
                    break-inside: avoid;
                }
                .card-side .text, .card-side .image {
                    grid-row: 1 / -1;
                    place-self: center;
                }
                .card-side .text {
                    font-size: 40px;
                    color: var(--dark-gray-6);
                }
                .card-side .image {
                    max-width: 100%;
                    max-height: 100%;
                }
                .card-side .signature {
                    display: flex;
                    align-items: center;
                    column-gap: 5px;
                    font-size: 10px;
                    font-weight: 500;
                    color: var(--dark-gray-4);
                }
                .card-side .signature img-ui {
                    height: 10px;
                    filter: grayscale(1);
                }
            `,
        ];
    }

    @property()
    cardA: string = "";

    @property()
    kindA: CardKind = "text";

    @property()
    cardB: string = "";

    @property()
    kindB: CardKind = "text";

    @property({ reflect: true })
    fonts?: string;

    firstUpdated() {
        if (this.fonts) {
            let fonts = this.fonts
                .split(",")
                .map(f => f.trim())
                .map(f => f.replaceAll("'", ""));
            loadFonts(fonts);
        }
    }

    render() {
        return html`
            <style>
                .text {
                    font-family: ${this.fonts}
                }
            </style>
            <fa-icon icon="fa-solid fa-scissors"></fa-icon>
            <div class="card">
                <div class="card-side">
                    ${
                        this.kindA === "text" ? (
                            html`<div class="text">${this.cardA}</div>`
                        ) : (
                            html`<img class="image" src=${this.cardA}>`
                        )
                    }
                    <div class="signature">
                        <img-ui path="core/page-header/logo.svg"></img-ui>.org
                    </div>
                </div>
                <div class="card-side">
                    ${
                        this.kindB === "text" ? (
                            html`<div class="text">${this.cardB}</div>`
                        ) : (
                            html`<img class="image" src=${this.cardB}>`
                        )
                    }
                    <div class="signature">
                        <img-ui path="core/page-header/logo.svg"></img-ui>.org
                    </div>
                </div>
            </div>
        `;
    }
}
