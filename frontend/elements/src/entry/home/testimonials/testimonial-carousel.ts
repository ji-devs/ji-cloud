import { CarouselSingle } from '@elements/core/carousel/single';
import { LitElement, html, css, customElement, property, query, internalProperty } from 'lit-element';
import { classMap } from 'lit-html/directives/class-map';
import "./testimonial-item";

@customElement('home-testimonial-carousel')
export class _ extends LitElement {
    static get styles() {
        return [css`
            :host {
                display: grid;
                grid-template-rows: repeat(2, auto);
                grid-template-columns: auto minmax(0, 1fr) auto;
                column-gap: 32px;
                row-gap: 40px;
            }
            .nav-arrow {
                border: 0;
                cursor: pointer;
                color: var(--main-yellow);
                background-color: transparent;
            }
            .nav-arrow:disabled {
                color: var(--main-red);
            }
            .points {
                display: flex;
                column-gap: 24px;
                justify-content: center;
                grid-column: 1 / -1;
            }
            .point {
                width: 16px;
                height: 16px;
                background-color: var(--main-red);
                border: 0;
                padding: 0;
                border-radius: 50%;
                cursor: pointer;
            }
            .point.active {
                background-color: #ffffff;
            }
        `];
    }

    back() {
        this.carousel.back();
    }

    forward() {
        this.carousel.forward();
    }

    @property({type: Number})
    pageCount: number = 1;

    @query("carousel-single")
    carousel!: CarouselSingle;

    @internalProperty()
    activePage: number = 0;

    private activePageChange(e: CustomEvent) {
        this.activePage = e.detail.activePage;
    }

    render() {
        return html`
            <button class="nav-arrow left" @click="${this.back}" ?disabled="${this.activePage === 0}">
                <icon-arrow direction="left"></icon-arrow>
            </button>

            <carousel-single @active-page-change="${this.activePageChange}">
                <slot></slot>
            </carousel-single>

            <button class="nav-arrow right" @click="${this.forward}" ?disabled="${this.activePage === this.pageCount-1}">
                <icon-arrow direction="right"></icon-arrow>
            </button>

            <div class="points">
                ${
                    Array.from(Array(this.pageCount).keys()).map(i => html`
                        <button
                            class="point ${classMap({"active": this.activePage === i})}"
                            @click="${() => this.carousel.goToPage(i)}"
                        ></button>
                    `)
                }
            </div>
        `;
    }
}
