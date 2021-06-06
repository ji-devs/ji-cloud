import { LitElement, html, css, customElement, property, internalProperty, query } from 'lit-element';
import { classMap } from 'lit-html/directives/class-map';
import "@elements/core/carousel/single";
import "@elements/core/icon-arrow";
import { CarouselSingle } from '@elements/core/carousel/single';
import { homeStyles } from '../styles';

const STR_TITLE = "What’s new?";

@customElement('home-whats-new')
export class _ extends LitElement {
    static get styles() {
        return [homeStyles, css`
            :host {
                background-color: #afcbf4;
                display: block;
                padding: 72px;
            }
            .width-holder {
                display: grid;
                row-gap: 40px;
            }
            h2 {
                text-align: center;
                color: #ffffff;
                font-size: 64px;
                font-weight: 900;
                margin: 0;
            }
            .carousel-wrapper {
                display: flex;
                align-items: center;
                column-gap: 26px;
                justify-content: center;
            }
            .nav-button {
                background-color: transparent;
                border: 0;
                cursor: pointer;
            }
            carousel-single {
                width: 80%;
            }
            .points {
                display: flex;
                column-gap: 24px;
                justify-content: center;
            }
            .point {
                width: 16px;
                height: 16px;
                background-color: #ffffff;
                border-radius: 50%;
                cursor: pointer;
            }
            .point.active {
                background-color: var(--main-yellow);
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
            <div class="width-holder">

                <h2>${STR_TITLE}</h2>

                <div class="carousel-wrapper">

                    <button class="nav-button left" @click="${this.back}" ?disabled="${this.activePage === 0}">
                        <icon-arrow direction="left"></icon-arrow>
                    </button>

                    <carousel-single @active-page-change="${this.activePageChange}">
                        <slot name="items"></slot>
                    </carousel-single>

                    <button class="nav-button right" @click="${this.forward}" ?disabled="${this.activePage === this.pageCount-1}">
                        <icon-arrow direction="right"></icon-arrow>
                    </button>

                </div>

                <div class="points">
                    ${
                        Array.from(Array(this.pageCount).keys()).map(i => html`
                            <div
                                class="point ${classMap({"active": this.activePage === i})}"
                                @click="${() => this.carousel.goToPage(i)}"
                            ></div>
                        `)
                    }
                </div>

            </div>
        `;
    }
}
