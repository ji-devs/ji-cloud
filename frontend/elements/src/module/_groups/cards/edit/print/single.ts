import { LitElement, html, css, customElement, property } from "lit-element";
import { CardKind } from "./types";

@customElement("module-card-print-single")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: inline-grid;
                    grid-template-rows: 20px 30vw;
                    width: 45vw;
                    border-right: var(--border);
                    margin-top: 20px;
                }
                fa-icon {
                    color: #a1a8ad;
                    justify-self: end;
                    transform: translate(10px, -10px) rotate(90deg);
                }
                .card {
                    border-top: var(--border);
                    border-bottom: var(--border);
                    border-left: var(--border);
                    display: grid;
                    grid-template-rows: 1fr min-content;
                    padding: 8px;
                    break-inside: avoid;
                }
                .card .text, .card .image {
                    grid-row: 1 / -1;
                    place-self: center;
                }
                .card .text {
                    font-size: 40px;
                    color: var(--dark-gray-6);
                }
                .card .image {
                    max-width: 100%;
                    max-height: 100%;
                }
                .card .signature {
                    display: flex;
                    align-items: center;
                    column-gap: 5px;
                    font-size: 10px;
                    font-weight: 500;
                    color: var(--dark-gray-4);
                }
                .card .signature img-ui {
                    height: 10px;
                    filter: grayscale(1);
                }
            `,
        ];
    }

    @property()
    card: string = "";

    @property()
    kind: CardKind = "text";

    render() {
        return html`
            <fa-icon icon="fa-solid fa-scissors"></fa-icon>
            <div class="card">
                ${
                    this.kind === "text" ? (
                        html`<div class="text">${this.card}</div>`
                    ) : (
                        html`<img class="image" src=${this.card}>`
                    )
                }
                <div class="signature">
                    <img-ui path="core/page-header/logo.svg"></img-ui> Jigzi.org
                </div>
            </div>
        `;
    }
}
