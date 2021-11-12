import {
    LitElement,
    html,
    css,
    customElement,
    query,
    internalProperty,
} from "lit-element";
import "@elements/core/carousel/single";
import "@elements/entry/home/home/icon-arrow";
import { CarouselSingle } from "@elements/core/carousel/single";

@customElement("carousel-single-example")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: grid;
                    justify-content: center;
                }
                .carousel-wrapper {
                    width: 800px;
                    display: grid;
                    grid-template-columns: 50px 50px;
                    justify-content: space-between;
                }
                .nav-button {
                    grid-row: 1;
                    display: grid;
                    place-content: center;
                    cursor: pointer;
                    border: 0;
                    font-size: 50px;
                    color: #387af4;
                    background-color: transparent;
                }
                .nav-button:disabled {
                    color: #a1a8ad;
                }
                .nav-button.left {
                    grid-column: 1;
                    z-index: 1;
                    background-image: linear-gradient(
                        to left,
                        #00000000 3%,
                        var(--light-blue-3) 80%
                    );
                }
                .nav-button.right {
                    grid-column: 2;
                    z-index: 1;
                    background-image: linear-gradient(
                        to right,
                        #00000000 3%,
                        var(--light-blue-3) 80%
                    );
                }
                carousel-single {
                    grid-column: 1 / -1;
                    grid-row: 1;
                }
                .dot {
                    display: flex;
                    justify-content: center;
                    column-gap: 6px;
                }
                .dot .active {
                    background-color: black;
                    color: white;
                }
                .card-wrapper {
                    display: grid;
                    place-content: center;
                }
            `,
        ];
    }

    back() {
        this.carousel.back();
    }

    forward() {
        this.carousel.forward();
    }

    private activePageChange(e: CustomEvent) {
        this.activePage = e.detail.activePage;
    }

    @query("carousel-single")
    carousel!: CarouselSingle;

    @internalProperty()
    activePage: number = 0;

    @internalProperty()
    cards: Array<string> = [
        "duplicate",
        "words-images",
        "begins-with",
        "lettering",
        "duplicate",
        "words-images",
        "begins-with",
        "lettering",
        "duplicate",
        "words-images",
        "begins-with",
        "lettering",
    ];

    render() {
        return html`
            <div class="carousel-wrapper">
                <button
                    class="nav-button left"
                    @click="${this.back}"
                    ?disabled="${this.activePage === 0}"
                >
                    <icon-arrow direction="left"></icon-arrow>
                </button>
                <carousel-single @active-page-change="${this.activePageChange}">
                    ${this.cards.map(
                        (cardMode) => html`
                            <div class="card-wrapper">
                                <choose-card mode="${cardMode}"></choose-card>
                            </div>
                        `
                    )}
                </carousel-single>
                <button
                    class="nav-button right"
                    @click="${this.forward}"
                    ?disabled="${this.activePage === this.cards.length - 1}"
                >
                    <icon-arrow direction="right"></icon-arrow>
                </button>
            </div>
            <div class="dot">
                ${this.cards.map(
                    (_, i) => html`
                        <button
                            class="${this.activePage === i ? "active" : ""}"
                            @click="${() => this.carousel.goToPage(i)}"
                        >
                            ${i + 1}
                        </button>
                    `
                )}
            </div>
        `;
    }
}
