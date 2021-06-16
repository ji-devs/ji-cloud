import { mediaUi } from '@utils/path';
import { LitElement, html, css, customElement, property, query, unsafeCSS } from 'lit-element';

const STR_SUBTITLE = "What do you want to do?";

type ScrollPosition = 'left' | 'right' | null;

const NAV_BUTTON_WIDTH = 80;

@customElement('choose-mode')
export class _ extends LitElement {
    static get styles() {
        return [css`
            :host {
                --item-width: 389px;
                --nav-button-width: ${NAV_BUTTON_WIDTH}px;
                --gap-width: 20px;
                --visible-items: 1;
            }
            @media only screen and (min-width: 1000px) {
                :host {
                    --visible-items: 2;
                }
            }
            @media only screen and (min-width: 1400px) {
                :host {
                    --visible-items: 3;
                }
            }
            @media only screen and (min-width: 1800px) {
                :host {
                    --visible-items: 4;
                }
            }
            @media only screen and (min-width: 2200px) {
                :host {
                    --visible-items: 5;
                }
            }

            :host {
                display: block;
                background-color: #d8e7fa;
                background-size: cover;
                display: grid;
                padding-top: 178px;
                box-sizing: border-box;
                height: 100vh;
                align-content: start;
                row-gap: 77px;
            }
            header {
                margin: 0 auto;
            }
            .title {
                font-family: Poppins;
                font-size: 40px;
                font-weight: 900;
                letter-spacing: -0.4px;
                text-align: left;
                color: var(--orange);
            }
            .subtitle {
                font-family: Poppins;
                font-size: 24px;
                letter-spacing: normal;
                text-align: left;
                color: var(--dark-gray-6);
            }
            .carousel-wrapper {
                display: grid;
                grid-template-columns: 50px 50px;
                justify-content: space-between;
            }
            .nav-button {
                grid-row: 1;
                display: grid;
                place-content: center;
                cursor: pointer;
            }
            .nav-button.left {
                grid-column: 1;
                z-index: 1;
                background-image: linear-gradient(to left, #00000000 3%, var(--light-blue-3) 80%);
            }
            .nav-button.right {
                grid-column: 2;
                z-index: 1;
                background-image: linear-gradient(to right, #00000000 3%, var(--light-blue-3) 80%);
            }
            .carousel-wrapper {
                /*
                    (visible-items * item-width)
                    +
                    ((visible-items - 1) * gap-width)
                    +
                    (nav-button-width * 2)
                */
                width: calc(
                    (var(--visible-items) * var(--item-width))
                    +
                    ((var(--visible-items) - 1) * var(--gap-width))
                    +
                    (var(--nav-button-width) * 2)
                );

                margin: 0 auto;
            }
            header {
                width: calc(
                    (var(--visible-items) * var(--item-width))
                    +
                    ((var(--visible-items) - 1) * var(--gap-width))
                );
            }
            .carousel {
                grid-row: 1;
                grid-column: 1 / -1;
                display: flex;
                column-gap: var(--gap-width);
                box-sizing: border-box;
                display: grid;
                grid-auto-flow: column;
                max-width: 100%;
                overflow-x: auto;
                grid-auto-columns: var(--item-width);
                padding: 0 var(--nav-button-width);
                scroll-snap-type: x mandatory;
                scrollbar-width: none; /* Firefox */
                scroll-padding-left: var(--nav-button-width);
            }
            .carousel::-webkit-scrollbar {
                display: none; /* Chromium and Safari */
            }
            .carousel:after {
                content: "";
                display: block;
                width: calc(var(--nav-button-width) - var(--gap-width));
            }
            .carousel ::slotted(*) {
                scroll-snap-align: start;
                scroll-snap-stop: always;
            };
        `];
    }

    @query(".carousel")
    private carousel!: HTMLElement;

    back() {
        this.carousel.scroll({
            left: this.carousel.scrollLeft - this.carousel.clientWidth,
            behavior: 'smooth',
        });
    }

    forward() {
        this.carousel.scroll({
            left: this.carousel.scrollLeft + this.carousel.clientWidth,
            behavior: 'smooth',
        });
    }

    getScrollPosition() : ScrollPosition {
        const carousel = this.carousel;
        if(!carousel || !carousel.scrollLeft) {
            return 'left';
        } else if (carousel.scrollWidth - carousel.scrollLeft - carousel.clientWidth - NAV_BUTTON_WIDTH < 1) {
            return 'right';
        } else {
            return null;
        }
    }

    @property({ type: String })
    scrollPosition: ScrollPosition = 'left';

    @property()
    title:string = "";

    private onScroll() {
        this.scrollPosition = this.getScrollPosition();
    }

    render() {
        const {title} = this;

        return html`
            <div class="">
                <!-- background: green;
                width: 20px;
                height: 30px;
                justify-self: end; -->
            </div>
            <header>
                <div class="title">${title}</div>
                <div class="subtitle">${STR_SUBTITLE}</div>
            </header>
            <div class="carousel-wrapper">
                <div class="nav-button left" @click="${this.back}">
                    <img-ui path="core/_common/chevron-left-${this.scrollPosition === 'left' ? 'grey' : 'blue'}-large.svg"></img-ui>
                </div>
                <div class="carousel" @scroll="${this.onScroll}">
                    <slot></slot>
                </div>
                <div class="nav-button right" @click="${this.forward}">
                    <img-ui path="core/_common/chevron-right-${this.scrollPosition === 'right' ? 'grey' : 'blue'}-large.svg"></img-ui>
                </div>
            </div>
        `;
    }
}
