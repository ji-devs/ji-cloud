import { LitElement, html, css, customElement, property } from "lit-element";

@customElement("flashcards-print")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    width: 100%;
                    display: block;
                }
                .cards {
                    --border: dashed 1px #a1a8ad;
                    text-align: center;
                }
                .card-wrapper {
                    display: inline-grid;
                    grid-template-rows: 20px 180px;
                    border-right: var(--border);
                    margin-top: 20px;
                }
                .card-wrapper fa-icon {
                    color: #a1a8ad;
                    justify-self: end;
                    transform: translate(10px, -10px) rotate(90deg);
                }
                .card {
                    display: grid;
                    grid-template-columns: 180px 1px 180px;
                    border-top: var(--border);
                    border-bottom: var(--border);
                    border-left: var(--border);
                }
                .card-side {
                    display: grid;
                    grid-template-rows: 1fr min-content;

                    padding: 8px;
                    break-inside: avoid;
                }
                .card-side .text {
                    font-size: 64px;
                    color: var(--dark-gray-6);
                    grid-row: 1 / -1;
                    place-self: center;
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
                .divider {
                    background-color: #a1a8ad;
                }
            `,
        ];
    }

    @property({ type: Array })
    cards: [string, string][] = [];

    render() {
        return html`
            <div class="cards">
                ${this.cards.map(card => html`
                    <div class="card-wrapper">
                        <fa-icon icon="fa-solid fa-scissors"></fa-icon>
                        <div class="card">
                            <div class="card-side">
                                <div class="text">${card[0]}</div>
                                <div class="signature">
                                    <img-ui path="core/page-header/logo.svg"></img-ui> Jigzi.org
                                </div>
                            </div>
                            <div class="divider"></div>
                            <div class="card-side">
                                <div class="text">${card[1]}</div>
                                <div class="signature">
                                    <img-ui path="core/page-header/logo.svg"></img-ui> Jigzi.org
                                </div>
                            </div>
                        </div>
                    </div>
                `)}
            </div>
        `;
    }
}
